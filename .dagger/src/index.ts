import {
  dag,
  Container,
  Directory,
  object,
  func,
  Secret,
  CacheSharingMode,
} from "@dagger.io/dagger";
import { Octokit } from "@octokit/rest";
import { createReadStream, rmSync, statSync } from "fs";
import { default as matter } from "gray-matter";
import { join } from "path";
import TOML from "@iarna/toml";

const WORK_DIR = "/src";
const DAGGER_DIR = join(WORK_DIR, ".dagger");
const CHANGELOGS_DIR = join(DAGGER_DIR, "changelogs");
const MAIN_BRANCH = "main";
const BASE_BINARY_NAME = "tohum";

type BumpType = "patch" | "minor" | "major";
type Version = `v${string}`;

type Contributor = {
  name: string;
  email: string;
  username?: string;
};

type Changelog = {
  breakingChanges: string[];
  fixes: string[];
  features: string[];
  bumps: BumpType[];
};

type CargoTOML = {
  package: {
    version: string;
  };
};

@object()
export class Pipeline {
  srcDir: Directory;
  container: Container;

  constructor(srcDir: Directory) {
    this.srcDir = srcDir;

    const cargoCache = dag.cacheVolume("cargo");
    const targetCache = dag.cacheVolume("target");

    this.container = dag
      .container()
      .from("rust:1.86.0-bookworm")
      .withMountedDirectory(WORK_DIR, this.srcDir)
      .withMountedCache(join(WORK_DIR, "target"), targetCache)
      .withMountedCache("/usr/local/cargo/registry", cargoCache)
      .withWorkdir(WORK_DIR);
  }

  @func()
  base(): Container {
    return this.container;
  }

  test(container: Container): Container {
    return container.withExec(["cargo", "test"]);
  }

  build(container: Container): Container {
    return container
      .withExec(["cargo", "build", "--release"])
      .withExec(["cp", "-R", "target", "/"]);
  }

  @func()
  publishCrate(cratesIoToken: Secret, container?: Container): Container {
    if (!container) {
      container = this.base();
    }

    // Test
    container = this.test(container);

    // Build
    container = this.build(container);

    // Publish
    return container
      .withSecretVariable("CARGO_REGISTRY_TOKEN", cratesIoToken)
      .withExec(["cargo", "publish"]);
  }

  @func()
  async releaseGithub(
    token: Secret,
    owner: string,
    repo: string,
    cratesIoToken?: Secret
  ): Promise<string> {
    const changelog = await parseChangelogs(this.container);

    if (changelog.bumps.length === 0) {
      throw new Error(`No changelog found, nothing to release`);
    }

    let nextVersion: Version = "v";
    const latestReleasedVersion = await latestVersionTag(this.container);
    const cargoToml = TOML.parse(
      await this.container.file("./Cargo.toml").contents()
    ) as CargoTOML;

    // This is the first release, take the initial version from cargo.toml
    if (!latestReleasedVersion) {
      nextVersion = `v${cargoToml.package.version}`;
    } else {
      /* const bump = biggestVersionBump(changelog.bumps);
      nextVersion = bumpVersion(latestReleasedVersion, bump); */
    }

    // Test
    this.container = this.test(this.container);

    // Build
    this.container = this.build(this.container);

    // Take out the exe file from cached directory
    /*  container = container.withExec([
      "cp",
      `${WORK_DIR}/target/release/tohum`,
      "/binary",
    ]); */

    // Update TOML file with the new version
    cargoToml.package.version = nextVersion.replaceAll("v", "");

    this.container = this.container
      // Write new version to the TOML file
      .withNewFile(`${WORK_DIR}/Cargo.toml`, TOML.stringify(cargoToml))

      // Clear changelogs and commit release
      .withExec([
        "sh",
        "-c",
        `rm -rf ${join(WORK_DIR, ".dagger", "changelogs", "*.md")}`,
      ])
      .withExec(["ls", "-la", ".dagger/changelogs"])
      .withExec([
        "git",
        "config",
        "--global",
        "user.email",
        "releasebot@github.com",
      ])
      .withExec(["git", "config", "--global", "user.name", "Release Bot"])
      .withExec(["git", "add", "."])
      .withExec(["git", "commit", "-m", `chore: release ${nextVersion}`]);

    // Publish to crates.io if the token is presented
    if (cratesIoToken) {
      this.container = this.publishCrate(cratesIoToken, this.container);
    }

    // Create git tag for the release commit
    const releaseCommitHash = (
      await this.container.withExec(["git", "rev-parse", "HEAD"]).stdout()
    ).trim();
    this.container = this.container
      .withExec(["git", "tag", nextVersion, releaseCommitHash])
      // Push back to the repository
      .withSecretVariable("GH_TOKEN", token)
      .withExec([
        "bash",
        "-c",
        `git push https://$GH_TOKEN@github.com/${owner}/${repo}.git ${MAIN_BRANCH}`,
      ]);

    // Create release
    const octokit = new Octokit({ auth: await token.plaintext() });
    const releaseInfo = await octokit.repos.createRelease({
      owner,
      repo: repo,
      tag_name: nextVersion,
      make_latest: "true",
      body: prepareReleaseText(changelog),
    });

    // Upload binary asset to the release
    const outputFilePath = await this.container
      .file(join("/target", "release", BASE_BINARY_NAME))
      .export(BASE_BINARY_NAME);

    const fileStat = statSync(outputFilePath);
    const fileData = createReadStream(outputFilePath);
    const assetInfo = await octokit.repos.uploadReleaseAsset({
      owner,
      repo,
      release_id: releaseInfo.data.id,
      name: `${BASE_BINARY_NAME}-linux-amd64`,
      headers: {
        "content-type": "application/octet-stream",
        "content-length": fileStat.size,
      },
      data: fileData as unknown as string,
    });
    rmSync(BASE_BINARY_NAME, { force: true });

    // TODO: Build for Windows and upload to the release as an asset
    // TODO: Build for Mac OS X and upload to the release as an asset

    return `Release ${nextVersion} is done`;
  }

  /**
   * Finds the contributors name, e-mail and GitHub usernames from commit history.
   * @param container
   * @param sinceVersion If not given, finds all the contributors since the initial commit
   */
  async getContributors(
    container: Container,
    sinceVersion?: `v${string}`
  ): Promise<Contributor[]> {
    const namesEmails = (
      await container
        .withExec([
          "bash",
          "-c",
          `git log ${
            sinceVersion
              ? `${sinceVersion}..HEAD` // We already released a version, only include contributors since that
              : "--reverse" // This is the first release, include all contributors from the initial commit.
          } --pretty=format:%an/%ae | sort | uniq`,
        ])
        .stdout()
    )
      .trim()
      .split("\n");

    // Parse contributors who contributed to new version
    const contributors = namesEmails.map<Promise<Contributor>>(async (line) => {
      const [name, email] = line.split("/");
      let username: string | undefined = undefined;

      // Try to find GitHub username from e-mail
      if (email !== undefined)
        try {
          const res = await fetch(`https://ungh.cc/users/find/${email}`, {
            method: "GET",
          });
          if (res.ok) {
            const userInfo = await res.json();
            username = userInfo.user.username;
          }
        } catch {}

      return { name, email, username };
    });

    // Wait until all username requests are completed
    return await Promise.all(contributors);
  }
}

/**
 * Gets the latest version tag from the git repository
 * @returns `undefined` if no valid tag found
 */
async function latestVersionTag(
  container: Container
): Promise<Version | undefined> {
  // List all of the tags that matches with the pattern which is `v*`
  const allTagsOutput = await container
    .withExec(["git", "tag", "--list", "v*"])
    .stdout();

  // Parse them
  const tags = allTagsOutput
    .split("\n")
    .map((tag) => tag.trim())
    .filter((tag) => tag.length > 0);

  // Get the latest one
  const latestVersionTag = tags.at(-1);

  return latestVersionTag as Version;
}

/**
 * Bumps the given version string
 * @returns New version string prefixed with `v` e.g `v1.2.3`
 */
function bumpVersion(version: string, bumpType: BumpType): Version {
  if (!version.startsWith("v")) {
    version = `v${version}`;
  }

  const match = /^v(\d+)\.(\d+)\.(\d+)$/.exec(version);

  if (!match) {
    throw new Error(`Invalid version: ${version}`);
  }

  let [, /* ignore whole match */ major, minor, patch] = match.map(Number);

  switch (bumpType) {
    case "patch":
      patch += 1;
      break;
    case "minor":
      minor += 1;
      patch = 0;
      break;
    case "major":
      major += 1;
      minor = 0;
      patch = 0;
      break;
  }

  return `v${major}.${minor}.${patch}`;
}

/**
 * Returns the biggest bump type from the given array.
 * Basically; major > minor > patch
 */
function biggestVersionBump(bumpTypes: BumpType[]): BumpType {
  if (bumpTypes.includes("major")) {
    return "major";
  }
  if (bumpTypes.includes("minor")) {
    return "minor";
  }

  return "patch";
}

/**
 * Builds a Markdown text for GitHub release information based on the `changelog`
 */
function prepareReleaseText(
  changelog: Changelog,
  contributors?: Contributor[]
) {
  let releaseText = "";

  if (changelog.breakingChanges.length > 0) {
    releaseText += "## ⚠️ Breaking Changes\n";
    for (const breakingChange of changelog.breakingChanges) {
      releaseText += `* ${breakingChange}\n`;
    }
  }

  if (changelog.features.length > 0) {
    releaseText += "## 🚀 Features\n";
    for (const feature of changelog.features) {
      releaseText += `* ${feature}\n`;
    }
  }

  if (changelog.fixes.length > 0) {
    releaseText += "## 🩹 Fixes\n";
    for (const fix of changelog.fixes) {
      releaseText += `* ${fix}\n`;
    }
  }

  if ((contributors?.length || 0) > 0) {
    releaseText += "## ❤️ Thank You\n";
    for (const contributor of contributors!) {
      releaseText += `* ${contributor.name} `;

      if (contributor.username) {
        releaseText += `@${contributor.username}`;
      }
      releaseText += "\n";
    }
  }

  return releaseText.trim();
}

/**
 * Parses the changelog files
 * @param container Container that has the base environment
 */
async function parseChangelogs(container: Container): Promise<Changelog> {
  const paths = await container.directory(CHANGELOGS_DIR).entries();
  const versionBumps = new Set<BumpType>();
  const features: string[] = [];
  const fixes: string[] = [];
  const breakingChanges: string[] = [];

  for (const path of paths) {
    const content = await container.file(join(CHANGELOGS_DIR, path)).contents();
    const info = matter(content);
    const bumpType: BumpType = info.data["bump-type"];

    // If bumpType is not defined in front matter of changelog file,
    // we simply ignore that changelog because it is invalid
    if (!bumpType) {
      continue;
    }

    // Based on the bumpType, add content of the markdown file to the array
    ({
      major: breakingChanges,
      minor: features,
      patch: fixes,
    })[bumpType].push(info.content.trim());

    // Even if we have multiple major, minor or patch changelogs, we need to
    // store them only once that's why we are using Set rather than a simple Array.
    versionBumps.add(bumpType);
  }

  return {
    bumps: [...versionBumps],
    features,
    fixes,
    breakingChanges,
  };
}
