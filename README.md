# maker

maker is a simple CLI tool that you can use to provision your new projects from the template that you've chosen. You can also create your own project templates to extend the functionality.

## Quickstart

First you need to install maker. You can do it either installing it from cargo registry or build your own.

Install from cargo registry:

```shell
# Not available yet :-)
```

Build and install from source:

```shell
git clone https://github.com/mkaramuk/maker.git
cd maker
cargo build --release
sudo ./install.sh
```

> For building from source, you must have Rust tool chain. You can achieve them by simply using [rustup](https://rustup.rs/)

Once you've done with the installation, you can create your first project by using one of the following templates:

| Description                                                  | Identifier |
| ------------------------------------------------------------ | ---------- |
| Go CLI application                                           | `go@cli`   |
| Vite React application with Tailwind and Redux support in TS | `react@ts` |

Use your desired template in the `init` command:

```shell
maker init go@cli my-super-duper-project
```

Ta-da! Now you have the project that ready to code!

For a custom path, you can use the `-p` or `--path` option:

```shell
maker init go@cli my-super-duper-project -p /path/to/your/project
```
