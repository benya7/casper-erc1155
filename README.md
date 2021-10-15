# Casper ERC1155

A library and example implementation of ERC1155 token for the Casper network.

You can see the documentation here [/docs/README.md](https://github.com/en0c-026/casper-erc1155/blob/master/docs/README.md).

The roadmap here [/docs/ROADMAP.md](https://github.com/en0c-026/casper-erc1155/blob/master/docs/ROADMAP.md).

This is the deploy hash of the example contract in Tesnet Casper:

[b10dd74764659c139ef83fb98e48dcd7c7b0c9f13304354be9723c6c2c80ab63](https://testnet.cspr.live/deploy/b10dd74764659c139ef83fb98e48dcd7c7b0c9f13304354be9723c6c2c80ab63)



## Install
Make sure the `wasm32-unknown-unknown` Rust target is installed.
```
make prepare
```

## Build Smart Contracts
To build the example ERC1155 contract and supporting test contracts:
```
make build-contracts
```

## Test
```
make test
```
