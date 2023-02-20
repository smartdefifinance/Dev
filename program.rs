use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{IsInitialized, Pack, Sealed},
    sysvar::{rent::Rent, Sysvar},
};

// Define the data structure for the account used to represent a loan
#[derive(Debug, Default, PartialEq)]
pub struct LoanAccount {
    pub is_initialized: bool,
    pub lender: Pubkey,
    pub borrower: Pubkey,
    pub amount: u64,
    pub collateral: u64,
}

// Implement the Pack trait to enable serialization of the loan account data
impl Pack for LoanAccount {
    const LEN: usize = 49;

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, 49];
        self.is_initialized.pack_into_slice(&mut output[0..1]);
        self.lender.pack_into_slice(&mut output[1..33]);
        self.borrower.pack_into_slice(&mut output[33..65]);
        self.amount.pack_into_slice(&mut output[65..73]);
        self.collateral.pack_into_slice(&mut output[73..81]);
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, 49];
        Ok(Self {
            is_initialized: input[0].into(),
            lender: Pubkey::new_from_array(*array_ref![input, 1, 32]),
            borrower: Pubkey::new_from_array(*array_ref![input, 33, 32]),
            amount: u64::from_le_bytes(*array_ref![input, 65, 8]),
            collateral: u64::from_le_bytes(*array_ref![input, 73, 8]),
        })
    }
}

// Implement the Sealed and IsInitialized traits to enable checks for whether an account is initialized
impl Sealed for LoanAccount {}
impl IsInitialized for LoanAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

// Define the program ID and the instruction type
#[derive(Debug)]
pub enum LendingBorrowingInstruction {
    InitLoan {
        lender_amount: u64,
        collateral_amount: u64,
    },
    BorrowLoan {
        borrower_amount: u64,
    },
}

// Define the entry point function for the program
#[entrypoint]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Lending-Borrowing Rust program entrypoint");

    // Parse the instruction data
    let instruction = LendingBorrowingInstruction::unpack(instruction_data)?;

    // Match the instruction and execute the appropriate logic
    match instruction {
        LendingBorrowingInstruction::InitLoan { lender_amount, collateral_amount } => {
            msg!("Instruction: InitLoan");

            // Check that the account passed in is owned by the program
            let account_info_iter = &mut accounts.iter();
            let loan_account_info = next_account_info(account_info_iter)?;
            if loan_account_info.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }

            // Check that the lender's account has enough funds to lend
            let lender_account_info = next_account_info(account_info_iter)?;
            let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
            if !rent.is_exempt(lender_account_info.lamports(), lender_account_info.data_len()) {
                return Err(ProgramError::AccountNotRentExempt);
            }
            if lender_account_info.lamports() < lender_amount {
                return Err(ProgramError::InsufficientFunds);
            }

            // Initialize the loan account with the lender's and borrower's public keys and the loan amount and collateral
            let loan_account = LoanAccount {
                is_initialized: true,
                lender: *lender_account_info.key,
                borrower: Pubkey::default(),
                amount: lender_amount,
                collateral: collateral_amount,
            };
            LoanAccount::pack_into_slice(&loan_account, &mut loan_account_info.data.borrow_mut())?;

            // Transfer the loan amount from the lender's account to the loan account
            **lender_account_info.try_borrow_mut_lamports()? -= lender_amount;
            **loan_account_info.try_borrow_mut_lamports()? += lender_amount;

            msg!("Loan initialized successfully");
            Ok(())
        },
        LendingBorrowingInstruction::BorrowLoan { borrower_amount } => {
            // BorrowLoan logic
            unimplemented!()
        },
    }
}
