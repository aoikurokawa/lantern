use pinocchio::{entrypoint, AccountView, Address, ProgramResult};

entrypoint!(process_instruction);

pinocchio_pubkey::declare_id!("Lann5fHSrV7sswqd5r55n5hN43DHcmveMZGpY7EufP5");

fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
