# Casper ERC1155

A library and example implementation of ERC1155 token for the Casper network.

You can see the documentation here [/docs/README.md](https://github.com/en0c-026/casper-erc1155/blob/master/docs/README.md).

The roadmap here [/docs/ROADMAP.md](https://github.com/en0c-026/casper-erc1155/blob/master/docs/ROADMAP.md).

This is the deploy hash of the example contract in Tesnet Casper:

[1595479224e3e66ece245fd21768c3c6e9eea06e3aee463e5939233c8befefc9](https://testnet.cspr.live/account/016406b2e0012197adc57cb4218ed176a56a954b688601767a447b270283d2986c)



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
