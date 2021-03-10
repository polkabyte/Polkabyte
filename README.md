# polkabyte

polkabyte client implementation for polkabyte, a Substrate compatible chain for smart contracts.

## Setup Environment

Install all the required dependencies with a single command

```bash
curl https://getpolkabyte.io -sSf | bash -s -- --fast
```

## Build

Once the development environment is set up, build the polkabyte client.

```bash
cargo build --release
```

## Usage

To run local dev node, do

```
cargo run --release -- --dev
```

To run test net 1, do

```
cargo run --release
```

or

```
cargo run --release -- --chain=./res/testnet-1.json
```
