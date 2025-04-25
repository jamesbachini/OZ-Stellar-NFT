```markdown
# OZ-Stellar-NFT

A simple NFT contract on Stellar Soroban using OpenZeppelin libraries, featuring a capped supply of 100 tokens.

Full tutorial here: [https://jamesbachini.com/openzeppelin-soroban-nfts/](https://jamesbachini.com/openzeppelin-soroban-nfts/)

This project demonstrates how to create a capped NFT collection on the Stellar Soroban network. It uses OpenZeppelin's token implementation with added logic to limit total supply to 100 tokens.

## Prerequisites

- [Rust](https://rust-lang.org)
- [Stellar CLI](https://github.com/stellar/stellar-cli/)

## Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/jamesbachini/OZ-Stellar-NFT.git
   ```

2. Build the contract:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

3. Deploy to Testnet:
   ```bash
   stellar contract deploy --wasm target/wasm32-unknown-unknown/release/ozstellarnft.wasm --source YOUR_SOURCE_ACCOUNT --network testnet
   ```

## Usage

### Minting NFTs
```rust
pub fn mint(e: &Env, to: Address) -> TokenId {
    let token_id: TokenId = Base::sequential_mint(e, &to);
    if token_id > 100 {
        panic!("Maximum supply reached");
    }
    token_id
}
```

## Metadata Setup
1. Upload images/metadata to IPFS (e.g., using [Pinata](https://pinata.cloud))
2. Set contract metadata in constructor:
```rust
Base::set_metadata(
    e,
    String::from_str(e, "ipfs://YOUR_METADATA_CID"),
    String::from_str(e, "SoroKittens"),
    String::from_str(e, "SKT"),
);
```

## Features
- Capped supply (100 tokens max)
- IPFS metadata support
- OpenZeppelin-based implementation

## License
This project is open-source and Licenced under MIT.

Based on [OpenZeppelin's example](https://github.com/OpenZeppelin/stellar-contracts).

## Links
- [Full Tutorial](https://jamesbachini.com/openzeppelin-soroban-nfts/)

- [Original OZ Example](https://github.com/OpenZeppelin/stellar-contracts/tree/main/examples/nft-sequential-minting)

```

*Note: Replace the tutorial link placeholder with your actual tutorial URL when available.*