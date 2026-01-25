<div align="center">
  <img src="./assets/logo_32_x_32.svg" alt="tohum logo" title="You should finish the projects that you have started" width=100/><br>
  <h1 style="margin-top: 10px; margin-bottom: 5px;">tohum</h1>
  <p style="margin-top: 0;"><em>A CLI tool for planting project seeds</em></p>
</div>

[![Crates.io Version](https://img.shields.io/crates/v/tohum)](https://crates.io/crates/tohum)
![GitHub License](https://img.shields.io/github/license/mkaramuk/tohum)
[![Issues](https://img.shields.io/github/issues/mkaramuk/tohum)](https://github.com/mkaramuk/tohum/issues)
![GitHub contributors](https://img.shields.io/github/contributors/mkaramuk/tohum)
[![StandWithPalestine](https://raw.githubusercontent.com/TheBSD/StandWithPalestine/main/badges/StandWithPalestine.svg)](https://github.com/TheBSD/StandWithPalestine/blob/main/docs/README.md)

<img src="./assets/meme.jpg" alt="meme" title="You should finish the projects that you have started" width=200/>
<hr />

"tohum" (/toˈhuːm/, meaning "seed" in Turkish) is a CLI tool for initializing new projects from pre-defined seeds (aka templates).

> ⚠️ **WARNING** ⚠️
> tohum is in its early stage of development, so expect breaking changes.

## Installation

### Dependencies

- [git](https://git-scm.com/)
- [openssl](https://github.com/openssl/openssl)

### cargo

Currently, tohum is only published on the cargo registry, which means you can simply use cargo to install it:

```sh
cargo install tohum
```

### Build from source

Another option (which is not very different from installing via cargo) is building from source. For this option you must have a Rust toolchain. You can install it by simply using [rustup](https://rustup.rs/).

```shell
git clone https://github.com/mkaramuk/tohum.git && cd tohum
cargo build --release
sudo ./install.sh # Installs the binary to /usr/local/bin/tohum (Linux only)
```

## Quickstart

Let's list all the seeds in the default silo:

```sh
$ tohum silo list

🌱 Found 1 seeds in the silo:
────────────────────────────────────────
  • @ts/cli
    Node.js project that configured for TypeScript. This seed uses "tsup" as the bundler.
    by Muhammed Karamuk
```

Now we know what are the available seeds that we can use. Pick one and initialize a new project. For example:

```sh
$ tohum plant @ts/cli my-super-cli-project
Project my-super-cli-project planted at my-super-cli-project from @ts/cli seed!
```

Congratulations! You've planted your first _tohum_ (seed)!

## Building seeds

A seed is a representation of your template project. It includes all the project files regardless of the framework or programming language in [tera](https://keats.github.io/tera/docs/) templating format and a special file called `.tohumrc`. This file includes all the necessary metadata information for the seed definition that is read by tohum.

### Structure

A typical seed directory looks like this:

```
my-awesome-seed/
├── .tohumrc
└── ... other project files
```

Here is an example `.tohumrc` file:

```jsonc
{
  // JSON schema definition. With this you may have intellisense in your IDE.
  "$schema": "https://raw.githubusercontent.com/mkaramuk/tohum/main/metadata.schema.json",

  // This name will be used in `tohum plant ...` command.
  "name": "my-seed",

  // An optional version for your seed. If omitted, set to 1.0.0 by default.
  "version": "1.0.0",

  // An optional description about what is your seed about.
  "description": "A description of my seed",

  // Optional metadata, tags.
  "tags": ["rust", "cli"],

  // Your seed must include at least one author.
  "authors": [
    {
      // Required
      "name": "Jane Doe",

      // Optional
      "email": "jane@example.com",

      // Optional
      "website": "https://janedoe.com",
    },
  ],

  // All the possible templating variables that can be used with this seed.
  "variables": {
    "project_name": {
      "type": "string",

      "description": "The name of the project",
    },
    "license": {
      // Type of the variable, required
      "type": "string",

      // Optional, if defined and the user does not explicitly defines
      // this variable in `tohum plant` command, then this value will be used.
      "default": "MIT",

      // Optional
      "description": "License type",

      // Optional, if set to `true` then "tohum plant" forces this variable
      // to be passed via `-v` flag.
      "required": true,
    },
  },
}
```

Some of the variables are auto defined by tohum and always available in your template context:

| Name         | Description                                   | Type                                                                                      |
| ------------ | --------------------------------------------- | ----------------------------------------------------------------------------------------- |
| project_name | Project name set inside `tohum plant` command | string                                                                                    |
| authors      | Authors array set inside `.tohumrc` file      | Array<{ name: string, email: string OR not available, website: string OR not available }> |

### Publishing

Your seeds need to be stored in a silo. A silo is simply a git repository that includes seeds. tohum uses this repository as the default silo (you can find seeds inside silo/ directory). You can structure your silo as you wish as long as it includes valid seeds, tohum will recursively scan the entire repo.

To publish your seeds you have two options:

- Create a silo (a new git repository) and tell people to use `tohum -s <your repo address>` so they will use your silo as the seed source.
- Open an issue in this repository to add your seed into the default silo.

Since tohum uses git to manage silos, you can even use local git repository as a silo by specifying their path via `-s <local git repo path>` flag.

## Contributing

We are open for all type of contributions including translations, adding and maintaining seeds, feature implementations and bug fixes.
