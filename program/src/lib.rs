use pinocchio::{entrypoint, AccountView, Address, ProgramResult};

entrypoint!(process_instruction);

pinocchio_pubkey::declare_id!("LanhbP4C2kBamFeD5KYHZ5Sfs5dquv6pP2KYEMX1Zkt");

fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
