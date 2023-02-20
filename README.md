------------------------------------------------------SMART FINANCE LENDING AND BORROWING PROTOCOL ------------------------------------------------------

--------------------------- PROTOCOL PROCESS ---------------------------
User management: Manage users and their balances. Implementing using a struct that contains a user's public key and their balance.

Loan management: Manage loans, including loan terms, interest rates, and payment schedules. Implement this using a struct that contains the lender's public key, borrower's public key, loan amount, interest rate, loan duration, and start time.

Loan request and approval: Need a way for borrowers to request loans and for lenders to approve or deny loan requests. Implementing this using a smart contract that validates the loan request and transfers funds from the lender's account to the borrower's account.

Loan repayment: Need a way for borrowers to repay loans and for lenders to receive loan repayments. Implementing this using a smart contract that tracks loan repayments and interest payments and transfers funds from the borrower's account to the lender's account.

Collateral management: Managing collateral implementing a secured lending protocol. Implementing this using a struct that contains the borrower's public key, the loan amount, and the collateral's public key.

Security measures: Implementing various security measures to prevent malicious actors from exploiting your protocol. Some of the security measures include access control, data validation, and data encryption.

--------------------------- DEPLOY SMART CONTRACT ---------------------------

1. Install the Solana command-line tools: Install the Solana command-line tools on local machine to deploy the smart contract. Following the installation instructions on the Solana documentation website.

2. Create a Solana wallet: create a Solana wallet to hold Solana tokens and pay for transaction fees. Using solana-keygen command to generate a new wallet.

3. Fund Solana wallet: Need to fund Solana wallet with SOL tokens to pay for transaction fees. On devnet simply using Solana faucet to airdrop via commandline. 

4. Write and compile the smart contract code: Smart contract code in Rust compiling it to a Solana program binary (.so file) using the Solana Rust SDK. 

5. **Deploy the smart contract to Solana



