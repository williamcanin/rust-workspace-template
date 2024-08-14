# Rust Workspace Template :crab:

This is a simple template for a [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html?highlight=worksp#creating-a-workspace)-based [Rust](https://www.rust-lang.org/) project.
Here are some simple examples to show what the project structure would look like using `cdylib` and `dylib`.

## Requirements

* Rust >= 1.30
* Make >= 3.0
* Git >= 2.0

### Development

1 - Clone:

```git clone https://github.com/williamcanin/rust-workspace-template.git```

2 - Enter the `examples/cdylib` or `examples/dylib` directory.

3 - Run the command below to see all commands:

```make```


### Production

1 - Run the command below:

```make release```

2 - On Windows, after compilation, add the `utils.dll` library path to the PATH or keep it in the same directory as the executable. On Linux, after compilation, add the `libutils.so` library path to the `LD_LIBRARY_PATH` environment variable.

3 - Remember that the project made in `dylib`, will need the Rust std dynamic library (std-<hash>.dll) to work. This library must be in the system path. Usually this dynamic library is in:

**Windows:** %RUSTUP_HOME%\toolchains\stable-x86_64-pc-windows-msvc\bin\std-<hash>.dll

## License

For license details, see: [LICENSE](https://github.com/williamcanin/rust-workspace-template/blob/main/LICENSE)