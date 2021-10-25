#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unnecessary_cast)]

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use sp_std::convert::{Into, TryFrom, TryInto};
use sp_runtime::{
    generic,
    traits::{
        BlakeTwo256, Verify, IdentifyAccount
    },
    MultiSignature, RuntimeDebug,
};
use crate::UncheckedExtrinsic;

pub type BlockNumber = u32;
/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Alias to the public key used for this chain, actually a `MultiSigner`. Like
/// the signature, this also isn't a fixed size when encoded, as different
/// cryptos have different size public keys.
pub type AccountPublic = <Signature as Verify>::Signer;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type AccountIndex = u32;
pub type Amount = i128;
pub type Balance = u128;
pub type Share = u128;
pub type Moment = u64;
pub type Nonce = u32;
pub type Index = u32;
pub type EraIndex = u32;
pub type AuctionId = u32;
pub type Hash = sp_core::H256;
pub type DigestItem = generic::DigestItem<Hash>;

pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type BlockId = generic::BlockId<Block>;
pub type SignedBlock = generic::SignedBlock<Block>;