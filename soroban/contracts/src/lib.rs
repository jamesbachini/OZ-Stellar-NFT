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
            String::from_str(e, "www.mytoken.com"),
            String::from_str(e, "My Token"),
            String::from_str(e, "TKN"),
        );
    }

    pub fn mint(e: &Env, to: Address) -> TokenId {
        Base::sequential_mint(e, &to)
    }
}

#[default_impl]
#[contractimpl]
impl NonFungibleToken for OZStellarNFT {
    type ContractType = Base;
}
