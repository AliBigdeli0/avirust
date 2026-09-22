# General

## Environment Variables

- `MED_REDIS_URL`: Required Redis connection URL (e.g., `redis://127.0.0.1:6379`).
- `MED_AMQ_ADDRESS`: Required ActiveMQ broker address (e.g., `tcp://localhost:61616`).
- `MED_MONGO_CONN_STR`: Required MongoDB connection string.
- `MED_WELL_ID`: Required ID of the well to process.
- `MED_STOMP_USERNAME`: STOMP authentication username; defaults to `admin`.
- `MED_STOMP_PASSWORD`: STOMP authentication password; defaults to `admin`.

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

## ServerDb argument for test in the real environment

```shell
mongodb://172.30.2.241:27017,172.29.48.77:27017,172.30.2.241:27018,172.30.2.237:27018/?replicaSet=rs0&connectTimeoutMS=5000&heartbeatFrequencyMS=2000
```

Entire execution command :

```shell
server_db.exe -i=2886263122 -d "mongodb://172.30.2.241:27017,172.29.48.77:27017,172.30.2.241:27018,172.30.2.237:27018/?replicaSet=rs0&connectTimeoutMS=5000&heartbeatFrequencyMS=2000&serverSelectionTimeoutMS=5000"
```

## Add Cargo to Vendor

```shell
cargo vendor
```
