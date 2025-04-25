#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use stellar_default_impl_macro::default_impl;
use stellar_non_fungible::{Balance, Base, NonFungibleToken, TokenId};

#[contract]
pub struct OZStellarNFT;

#[contractimpl]
impl OZStellarNFT {
    pub fn __constructor(e: &Env) {
        Base::set_metadata(
            e,
            String::from_str(e, "ipfs://bafkreigjf3tymofuq5vepmlijsglf65qprsiwykkkz6ipdgxv6fnxcje4e"),
            String::from_str(e, "SoroKittens"),
            String::from_str(e, "SKT"),
        );
    }

    pub fn mint(e: &Env, to: Address) -> TokenId {
        let token_id: TokenId = Base::sequential_mint(e, &to);
        if token_id > 100 {
            panic!("Maximum minted already");
        }
        token_id
    }
}

#[default_impl]
#[contractimpl]
impl NonFungibleToken for OZStellarNFT {
    type ContractType = Base;
}
