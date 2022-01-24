use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // 0 = create_campaign, 1 = withdraw, 2 = donate
    if instruction_data[0] == 0 {
        return create_campaign(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 2 {
        return withdraw (
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        );
    } else if instruction_data[0] == 2 {
        return donate (
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()]
        )
    }
    Ok(())
}

entrypoint!(process_instruction);