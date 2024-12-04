use {
    mollusk_svm::Mollusk,
    solana_sdk::{account::AccountSharedData, pubkey::Pubkey},
};

pub const ID: Pubkey = solana_sdk::pubkey!("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

pub const ELF: &[u8] = include_bytes!("elf/mpl_core.so");

pub fn add_program(mollusk: &mut Mollusk) {
    // Loader v2
    mollusk.add_program_with_elf_and_loader(
        &ID,
        ELF,
        &mollusk_svm::program::loader_keys::LOADER_V2,
    );
}

pub fn account() -> AccountSharedData {
    // Loader v2
    mollusk_svm::program::create_program_account_loader_v2(ELF)
}

/// Get the key and account for the SPL Token program.
pub fn keyed_account() -> (Pubkey, AccountSharedData) {
    (ID, account())
}
