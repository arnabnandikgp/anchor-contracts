
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info, entrypoint::{ProgramResult, __AccountInfo}, example_mocks::solana_sdk::system_instruction, program::invoke, pubkey::Pubkey
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct NameAccount {
    pub name: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum NameInstructions {
    Iniatialize(String),
    Update(String),
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[__AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let iter = &mut accounts.iter();

    let payer = next_account_info(iter).unwrap();
    let name_account = next_account_info(iter).unwrap();
    let system_program_info = next_account_info(iter).unwrap();

    match NameInstructions::try_from_slice(instruction_data)? {
        NameInstructions::Iniatialize(name) => {
            let create_ix = system_instruction::create_account(
                payer.key,
                name_account.key,
                1000000000,
                82,
                program_id,
            );
            invoke(
                &create_ix,
                &[
                    payer.clone(),
                    name_account.clone(),
                    system_program_info.clone(),
                ],
            )?;

            let name_account_data = NameAccount { name };
            name_account_data.serialize(&mut *name_account.data.borrow_mut())?;
        }
        NameInstructions::Update(name) => {
            let mut name_account_data = NameAccount::try_from_slice(&name_account.data.borrow())?;
            name_account_data.name = name;
            name_account_data.serialize(&mut *name_account.data.borrow_mut())?;
        }
    }

    Ok(())
}
