# Rust Workspace Template

This is a simple template for a [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html?highlight=worksp#creating-a-workspace)-based [Rust](https://www.rust-lang.org/) project.
There are a few simple examples to show what the project structure would look like.

## Requirements

* Rust >= 1.30
* Make >= 3.0
* Git >= 2.0

## Usage

### Development

1 - Clone:

```git clone https://github.com/williamcanin/rust-workspace-template.git```

2 - Run the command below to see all commands:

```make```

### Production

1 - Run the command below:

```make release```

2 - On Windows, after compilation, add the `fs.dll` library path to the PATH or keep it in the same directory as the executable. On Linux, after compilation, add the `libfs.so` library path to the `LD_LIBRARY_PATH` environment variable.

## License

For license details, see: [LICENSE](https://github.com/williamcanin/rust-workspace-template/blob/main/LICENSE)
