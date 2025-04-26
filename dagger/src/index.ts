import {
  dag,
  Container,
  Directory,
  object,
  func,
  Secret,
} from "@dagger.io/dagger";
import { Octokit } from "@octokit/rest";
import { createReadStream, readdirSync, readFileSync, statSync } from "fs";
import { default as matter } from "gray-matter";
import { join } from "path";
import * as toml from "toml";

type BumpType = "patch" | "minor" | "major";
type ReleaseCommit = {
  commitHash: string;
  bumpType?: BumpType;
  changeLog?: string;
};

const WORK_DIR = "/src";

@object()
export class Pipeline {
  srcDir: Directory;

  constructor(srcDir: Directory) {
    this.srcDir = srcDir;
  }

  @func()
  base(): Container {
    return dag
      .container()
      .from("rust:1.86.0-bookworm")
      .withMountedDirectory(WORK_DIR, this.srcDir)
      .withMountedCache(join(WORK_DIR, "target"), dag.cacheVolume("target"))
      .withWorkdir(WORK_DIR);
  }

  @func()
  test(container: Container): Container {
    return container.withExec(["cargo", "test"]);
  }

  @func()
  build(container: Container): Container {
    return container.withExec(["cargo", "build", "--release"]);
  }

  bumpVersion(version: string, bumpType: BumpType): string {
    if (!version.startsWith("v")) {
      version = `v${version}`;
    }

    const match = /^v(\d+)\.(\d+)\.(\d+)$/.exec(version);

    if (!match) {
      throw new Error(`Invalid version: ${version}`);
    }

    let [, major, minor, patch] = match.map(Number);

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

  async getLatestVersionTag(container: Container): Promise<string | undefined> {
    const allTagsOutput = await container
      .withExec(["git", "tag", "--list", "v*"])
      .stdout();
    const tags = allTagsOutput
      .split("\n")
      .map((tag) => tag.trim())
      .filter((tag) => tag.length > 0);
    const latestVersionTag = tags.at(-1);

    return latestVersionTag;
  }

  async getReleaseCommit(container: Container): Promise<ReleaseCommit> {
    const commitDetails = await container
      .withExec(["git", "log", "-1", "--pretty=format:%H\n%s%n%b", "--quiet"])
      .stdout();

    const [commitHash, commitTitle, changeLog] = commitDetails.split("\n", 2);
    const match = commitTitle.match(/chore(:|\((patch|minor|major)\))/);

    if (!match) {
      throw new Error(`Latest commit is not a release commit`);
    }

    return {
      commitHash,
      changeLog,
      bumpType: match[2] as BumpType | undefined,
    };
  }

  readVersionFromTOML(tomlFileContent: string) {
    const parsedToml = toml.parse(tomlFileContent);
    return parsedToml.package.version;
  }

  getBiggestBump(bumpTypes: BumpType[]): BumpType {
    if (bumpTypes.includes("major")) {
      return "major";
    }
    if (bumpTypes.includes("minor")) {
      return "minor";
    }

    return "patch";
  }

  @func()
  async releaseGithub(
    token: Secret,
    owner: string,
    repo: string
  ): Promise<string> {
    let container = this.base();

    const changelogsPath = join(WORK_DIR, "dagger", "changelogs");
    const paths = await container.directory(changelogsPath).entries();

    const versionBumps = new Set<BumpType>();
    const features: string[] = [];
    const patches: string[] = [];
    const breakingChanges: string[] = [];

    for (const path of paths) {
      const content = await container
        .file(join(changelogsPath, path))
        .contents();
      const info = matter(content);
      const bumpType: BumpType = info.data["bump-type"];

      if (!bumpType) {
        continue;
      }

      ({
        major: breakingChanges,
        minor: features,
        patch: patches,
      })[bumpType].push(info.content.trim());

      versionBumps.add(bumpType);
    }

    if (versionBumps.size === 0) {
      throw new Error(`No changelog found`);
    }

    let latestVersion = await this.getLatestVersionTag(container);

    // This is the first release, take the initial version from cargo.toml
    if (!latestVersion) {
      const tomlVersion = this.readVersionFromTOML(
        await container.file("./Cargo.toml").contents()
      );
      latestVersion = `v${tomlVersion}`;
    } else {
      const bumpType = this.getBiggestBump([...versionBumps]);
      latestVersion = this.bumpVersion(latestVersion, bumpType);
    }

    // Test
    container = await this.test(container);

    // Build and take out the executable file from cached directory so we can access to it
    container = await this.build(container).withExec([
      "cp",
      `${WORK_DIR}/target/release/tohum`,
      "/binary",
    ]);

    // Clear changelogs and commit release
    container = container
      .withExec(["rm", "-rf", "dagger/changelogs/*.md"])
      .withExec(["git", "add", "."])
      .withExec(["git", "commit", "-m", "chore: release"]);

    const latestCommitHash = (
      await container.withExec(["git", "rev-parse", "HEAD"]).stdout()
    ).trim();

    // Create git tag for the new version
    container = container.withExec([
      "git",
      "tag",
      latestVersion,
      latestCommitHash,
    ]);

    // Push back to the repository
    container = container.withExec([
      "git",
      "push",
      `https://${token}@github.com/${repo}.git`,
      "main",
    ]);

    // Create release
    const octokit = new Octokit({ auth: await token.plaintext() });

    // Build release description
    let releaseText = "";

    if (breakingChanges.length > 0) {
      releaseText += "# Breaking Changes\n";
      for (const breakingChange of breakingChanges) {
        releaseText += `* ${breakingChange}\n`;
      }
    }

    if (features.length > 0) {
      releaseText += "# Features\n";
      for (const feature of features) {
        releaseText += `* ${feature}\n`;
      }
    }

    if (patches.length > 0) {
      releaseText += "# Fixes\n";
      for (const patch of patches) {
        releaseText += `* ${patch}\n`;
      }
    }

    const releaseInfo = await octokit.repos.createRelease({
      owner,
      repo: repo,
      tag_name: latestVersion,
      make_latest: "true",
      body: releaseText,
    });

    // Upload binary asset to the release
    const outputFilePath = await container.file("/binary").export("./tohum");
    const fileStat = statSync(outputFilePath);
    const fileData = createReadStream(outputFilePath);

    const assetInfo = await octokit.repos.uploadReleaseAsset({
      owner,
      repo,
      release_id: releaseInfo.data.id,
      name: "tohum-linux-amd64",
      headers: {
        "content-type": "application/octet-stream",
        "content-length": fileStat.size,
      },
      data: fileData as unknown as string,
    });

    // TODO: Create a release for windows
    // TODO: Create a release for mac os x

    return assetInfo.data.name;
  }
}
