# tohum

"tohum" (which means "seed" in Turkish) is a CLI tool that you can use to initialize your new projects from a pre-written template that you've chosen. You can also implement your own project templates to extend the functionality.

> tohum is under development. Expect breaking changes

## Quickstart

First you need to install tohum. You can do it either installing it from cargo registry or build your own.

Install from cargo registry:

```shell
# WIP
```

Build and install from source:

```shell
git clone https://github.com/mkaramuk/tohum.git
cd tohum
cargo build --release
sudo ./install.sh
```

> For building from source, you must have Rust tool chain. You can achieve them by simply using [rustup](https://rustup.rs/)

Once you've done with the installation, you can create your first project by using one of the following templates:

| Description                                                  | Identifier |
| ------------------------------------------------------------ | ---------- |
| Node.js TypeScript (tsup bundler + eslint)                   | `ts@node`  |
| Go CLI application                                           | `cli@go`   |
| Vite React application with Tailwind and Redux support in TS | `react@ts` |

Use your desired template in the `init` command:

```shell
tohum init cli@go my-super-duper-project
```

Ta-da! Now you have the project that ready to code!

For a custom path, you can use the `-p` or `--path` option:

```shell
tohum init cli@go my-super-duper-project -p /path/to/your/project
```
