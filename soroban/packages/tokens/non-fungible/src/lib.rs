//! # Non-Fungible Token Contract Module.
//!
//! Implements utilities for handling non-fungible tokens in a Soroban contract.
//!
//! This module provides essential storage functionalities required for managing
//! balances, approvals, and transfers of non-fungible tokens.
//!
//! ## Design Overview
//!
//! This module is structured to provide flexibility to developers by splitting
//! functionalities into higher-level and lower-level operations:
//!
//! - **High-Level Functions**: These include all necessary checks,
//!   verifications, authorizations, state-changing logic, and event emissions.
//!   They simplify usage by handling core logic securely. Users can directly
//!   call these functions for typical token operations without worrying about
//!   implementation details.
//!
//! - **Low-Level Functions**: These offer granular control for developers who
//!   need to compose their own workflows. Such functions expose internal
//!   mechanisms and require the caller to handle verifications and
//!   authorizations manually.
//!
//! By offering this dual-layered approach, developers can choose between
//! convenience and customization, depending on their project requirements.
//!
//! ## Structure
//!
//! The base module includes:
//!
//! - Transfers
//! - Owner and Approval management
//! - Metadata management (`name`, `symbol`, and `token_uri`)
//!
//! The following optional extensions are available:
//!
//! - *Burnable* enables token holders to destroy their non-fungible tokens.
//! - *Enumerable* allows for enumeration of all the token IDs in the contract
//!   as well as all the token IDs owned by each account.
//! - *Consecutive* is useful for efficiently minting multiple tokens in a
//!   single transaction.
//!
//! ## Compatibility and Compliance
//!
//! The ERC-721 interface is adapted to Stellar Ecosystem, facilitating
//! cross-ecosystem familiarity and ease of use, with the following differences:
//!
//! - `transfer()` function is made available due to consistency with Fungible
//!   Token interface, and also it is a simpler (thus, cheaper and faster)
//!   version of `transferFrom()`, which may become handy depending on the
//!   context.
//! - `safeTransfer` mechanism is not present in the base module, (will be
//!   provided as an extension)
//! - `name()`, `symbol()` and `token_uri()` functionalities are made available
//!   to be consistent with fungible tokens as well.
//!
//!
//! ## Notes for Developers
//!
//! - **Security Considerations**: While high-level functions handle necessary
//!   checks, users of low-level functions must take extra care to ensure
//!   correctness and security.
//! - **Composable Design**: The modular structure encourages developers to
//!   extend functionality by combining provided primitives or creating custom
//!   extensions.
#![no_std]

mod extensions;
mod non_fungible;
mod overrides;
mod storage;
mod utils;

pub use extensions::{burnable, consecutive, enumerable};
pub use non_fungible::{
    emit_approve, emit_approve_for_all, emit_transfer, Balance, NonFungibleToken,
    NonFungibleTokenError, TokenId,
};
pub use overrides::*;
pub use storage::{ApprovalData, ApprovalForAllData, StorageKey};
pub use utils::sequential;

mod test;
