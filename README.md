# boilit

boilit is a configurable cli tool to help you quickly create files passed on patterns of your project

## Getting started

### Via Crates.io

```shell
cargo install boilit
```

### Via GitHub Release

Download the correct file depending on your OS.

Create a `boilit.toml` file with your custom file pattern definitions.

Once created you can define your file names, contents, and other parameters into your config

## CLI

Use `boilit --help` to view available commands at any point

## Releasing

1. Bump the version in `Cargo.toml`
2. Create a new release via GitHub
3. GHA will create the windows and linux binary/exe
4. Run `buildMacOS.sh` on a mac
5. Upload the tar.gz to the release page
