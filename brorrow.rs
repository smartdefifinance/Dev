LendingBorrowingInstruction::BorrowLoan { borrower_amount } => {
    msg!("Instruction: BorrowLoan");

    // Check that the account passed in is owned by the program
    let account_info_iter = &mut accounts.iter();
    let loan_account_info = next_account_info(account_info_iter)?;
    if loan_account_info.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check that the loan account has been initialized
    let loan_account = LoanAccount::unpack_from_slice(&loan_account_info.data.borrow())?;
    if !loan_account.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }

    // Check that the borrower's account has enough funds to borrow
    let borrower_account_info = next_account_info(account_info_iter)?;
    let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
    if !rent.is_exempt(borrower_account_info.lamports(), borrower_account_info.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }
    if borrower_account_info.lamports() < borrower_amount {
        return Err(ProgramError::InsufficientFunds);
    }

    // Transfer the collateral from the loan account to the borrower's account
    **loan_account_info.try_borrow_mut_lamports()? -= loan_account.collateral;
    **borrower_account_info.try_borrow_mut_lamports()? += loan_account.collateral;

    // Transfer the borrowed amount from the loan account to the borrower's account
    **loan_account_info.try_borrow_mut_lamports()? += borrower_amount;
    **borrower_account_info.try_borrow_mut_lamports()? -= borrower_amount;

    // Update the loan account with the borrower's public key and set is_initialized to false
    let mut updated_loan_account = LoanAccount {
        is_initialized: false,
        lender: loan_account.lender,
        borrower: *borrower_account_info.key,
        amount: loan_account.amount,
        collateral: loan_account.collateral,
    };
    LoanAccount::pack_into_slice(&updated_loan_account, &mut loan_account_info.data.borrow_mut())?;

    msg!("Loan borrowed successfully");
    Ok(())
},
