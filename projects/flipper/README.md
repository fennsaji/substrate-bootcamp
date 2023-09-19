## Installation and Build
```
rustup update stable
rustup install 1.69
rustup default 1.69
rustup component add rust-src --toolchain 1.69
rustup target add wasm32-unknown-unknown --toolchain 1.69
cargo contract build
```

## Run
### Create/Deploy Contract
```
cargo contract instantiate --constructor new --args "false" --suri //Alice --salt $(date +%s)
```

### Query Contract
```
cargo contract call --contract 5FX58SrkKddUS8MHN38mdcAxGWbTtzhwYgtUEUj8ccHrnnEW --message get --suri //Alice --dry-run
```

### Call Contract
```
cargo contract call --contract 5FX58SrkKddUS8MHN38mdcAxGWbTtzhwYgtUEUj8ccHrnnEW --message flip --suri //Alice
```