# General

## Build Windows .exe File From Linux

Before anything just pay attension to run :

```shell
rustup target add x86_64-pc-windows-gnu
```

then add the linker to cargo using folowing

```yaml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

to build for windows you can run following command (in this example I want to generate server_db projects .exe):

```shell
cargo build --target=x86_64-pc-windows-gnu -p server_db --release
```

## Clap Argument with space

to add argument with space in the clap library :

```shell
cargo run -p server_db -- -n well name

# NOTE: this is incorrect
cargo build -- -n=\"well name\"
```
