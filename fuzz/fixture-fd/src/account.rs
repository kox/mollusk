//! An account with an address: `(Pubkey, AccountSharedData)`.

use {
    super::proto::{AcctState as ProtoAccount, SeedAddress as ProtoSeedAddress},
    solana_sdk::{
        account::{Account, AccountSharedData},
        pubkey::Pubkey,
    },
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SeedAddress {
    /// The seed address base (32 bytes).
    pub base: Vec<u8>,
    /// The seed path  (<= 32 bytes).
    pub seed: Vec<u8>,
    /// The seed address owner (32 bytes).
    pub owner: Vec<u8>,
}

impl From<ProtoSeedAddress> for SeedAddress {
    fn from(value: ProtoSeedAddress) -> Self {
        let ProtoSeedAddress { base, seed, owner } = value;
        Self { base, seed, owner }
    }
}

impl From<SeedAddress> for ProtoSeedAddress {
    fn from(value: SeedAddress) -> Self {
        let SeedAddress { base, seed, owner } = value;
        ProtoSeedAddress { base, seed, owner }
    }
}

impl From<ProtoAccount> for (Pubkey, AccountSharedData, Option<SeedAddress>) {
    fn from(value: ProtoAccount) -> Self {
        let ProtoAccount {
            address,
            owner,
            lamports,
            data,
            executable,
            rent_epoch,
            seed_addr,
        } = value;

        let pubkey_bytes: [u8; 32] = address.try_into().expect("Invalid bytes for pubkey");
        let pubkey = Pubkey::new_from_array(pubkey_bytes);

        let owner_bytes: [u8; 32] = owner.try_into().expect("Invalid bytes for owner");
        let owner = Pubkey::new_from_array(owner_bytes);

        (
            pubkey,
            AccountSharedData::from(Account {
                data,
                executable,
                lamports,
                owner,
                rent_epoch,
            }),
            seed_addr.map(Into::into),
        )
    }
}

impl From<(Pubkey, AccountSharedData, Option<SeedAddress>)> for ProtoAccount {
    fn from(value: (Pubkey, AccountSharedData, Option<SeedAddress>)) -> Self {
        let Account {
            lamports,
            data,
            owner,
            executable,
            rent_epoch,
        } = value.1.into();

        ProtoAccount {
            address: value.0.to_bytes().to_vec(),
            owner: owner.to_bytes().to_vec(),
            lamports,
            data,
            executable,
            rent_epoch,
            seed_addr: value.2.map(Into::into),
        }
    }
}

impl From<(Pubkey, AccountSharedData)> for ProtoAccount {
    fn from(value: (Pubkey, AccountSharedData)) -> Self {
        let Account {
            lamports,
            data,
            owner,
            executable,
            rent_epoch,
        } = value.1.into();

        ProtoAccount {
            address: value.0.to_bytes().to_vec(),
            owner: owner.to_bytes().to_vec(),
            lamports,
            data,
            executable,
            rent_epoch,
            seed_addr: None,
        }
    }
}