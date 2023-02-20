struct User {
    pub address: Pubkey,
    pub balance: u64,
}

struct Loan {
    pub lender: Pubkey,
    pub borrower: Pubkey,
    pub amount: u64,
    pub interest_rate: u64,
    pub duration: u64,
    pub start_time: u64,
}

struct LendingProtocol {
    pub users: HashMap<Pubkey, User>,
    pub loans: HashMap<Pubkey, Loan>,
}

impl LendingProtocol {
    pub fn new() -> Self {
        LendingProtocol {
            users: HashMap::new(),
            loans: HashMap::new(),
        }
    }

    pub fn get_user_balance(&self, address: &Pubkey) -> u64 {
        match self.users.get(address) {
            Some(user) => user.balance,
            None => 0,
        }
    }

    pub fn create_loan(
        &mut self,
        lender: &Pubkey,
        borrower: &Pubkey,
        amount: u64,
        interest_rate: u64,
        duration: u64,
        start_time: u64,
    ) -> Result<(), &'static str> {
        let lender_balance = self.get_user_balance(lender);
        if lender_balance < amount {
            return Err("Lender does not have sufficient balance");
        }

        let loan = Loan {
            lender: *lender,
            borrower: *borrower,
            amount: amount,
            interest_rate: interest_rate,
            duration: duration,
            start_time: start_time,
        };
        self.loans.insert(*lender, loan);

        let lender_user = self.users.get_mut(lender).unwrap();
        lender_user.balance -= amount;

        let borrower_user = self.users.entry(*borrower).or_insert(User {
            address: *borrower,
            balance: 0,
        });
        borrower_user.balance += amount;

        Ok(())
    }

    pub fn repay_loan(&mut self, lender: &Pubkey, borrower: &Pubkey, amount: u64) -> Result<(), &'static str> {
        let loan = match self.loans.get(lender) {
            Some(l) if l.borrower == *borrower => l,
            _ => return Err("Loan not found"),
        };

        let now = solana_program::clock::UnixTimestamp::now().into();
        let elapsed_time = now - loan.start_time;
        let interest_owed = (loan.amount * loan.interest_rate * elapsed_time) / (365 * 86400);

        let total_owed = loan.amount + interest_owed;

        if amount > total_owed {
            return Err("Amount exceeds amount owed");
        }

        let lender_user = self.users.get_mut(lender).unwrap();
        lender_user.balance += amount;

        let borrower_user = self.users.get_mut(borrower).unwrap();
        borrower_user.balance -= amount;

        if amount == total_owed {
            self.loans.remove(lender);
        } else {
            let new_loan = Loan {
                lender: *lender,
                borrower: *borrower,
                amount: total_owed - amount,
                interest_rate: loan.interest_rate,
                duration: loan.duration - elapsed_time,
                start_time: now,
            };
            self.loans.insert(*lender, new_loan);
        }

        Ok(())
    }
}
