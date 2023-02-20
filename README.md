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

--------------------------------------------

The user interface has two sections: one for creating a loan, and one for borrowing a loan. In the create loan section, the user can input the lender amount, collateral amount, and lender public key, and then click the Create Loan button to create a new loan account on the Solana blockchain. The loan public key is then displayed to the user.

In the borrow loan section, the user can input the borrower amount, borrower public key, and loan public key. When the user clicks the Borrow Loan button, the frontend will call the borrowLoan function from the lendingBorrowing module to send a transaction to the Solana blockchain that borrows the specified amount from the loan.


------------------------------------------- HOW TO INSTALL - 
************ To install the dependencies for the frontend user interface, you can follow these steps: *****************

    Ensure that you have Node.js and npm installed on your machine.
    Open a terminal or command prompt.
    Navigate to the directory where you want to create your project.
    Run the following command to create a new React app:
    
    " npx create-react-app lending-borrowing-ui --template typescript "
    This will create a new React app with TypeScript support in a directory called lending-borrowing-ui.
    
    1. Change to the new directory:
       cd lending-borrowing-ui
    2. Install the @solana/web3.js package and the borsh package by running:
       npm install @solana/web3.js borsh
    3. Replace the App.tsx file in the src directory with the code I provided.
    4. Start the app by running: 
       npm start
    5. This should open a web browser and display the user interface for the lending and borrowing protocol.  

********* Rust Installation *****************

    First, install Rust by following the instructions on the official Rust website: https://www.rust-lang.org/tools/install.
    Verify that Rust is installed correctly by running the following command in a terminal or command prompt:
    rustc --version

********* Solana Setup *****************

Install the Solana Command Line Interface (CLI) by following the instructions on the official Solana website: https://docs.solana.com/cli/install-solana-cli-tools.
Verify that the Solana CLI is installed correctly by running the following command in a terminal or command prompt:
solana --version

********* Solana Program Deployment *****************
1. Create a new Solana program by running the following command in a terminal or command prompt:
cargo new my-program

2. Add the following dependencies to your Cargo.toml file:
[dependencies]
solana-program = "1.7.0"
borsh = "0.8.4"
This will allow you to use the Solana program libraries and the Borsh serialization library.

3. Create a new Solana program by running the following command:
solana program deploy target/deploy/my_program.so
This will compile Rust program and deploy it to the Solana blockchain.

Once the program is deployed, can call its functions using the Solana CLI or through a custom user interface.

------------------------------------------- NOTES -------------------------------------------


1. that in the InitLoan case, we first check that the account passed in is owned by the program, and then check that the lender's account has enough funds to lend. We use the Rent sysvar to ensure that the lender's account has enough rent-exempt space to store the account data, and then check that the account has enough lamports to cover the lender_amount. We then initialize the loan account with the lender's and borrower's public keys and the loan amount and collateral, and transfer the loan amount from the lender's account to the loan account.

2. In the BorrowLoan case, we first check that the account passed in is owned by the program, and then check that the loan account has been initialized. We use the Rent sysvar to ensure that the borrower's account has enough rent-exempt space to store the account data, and then check that the account has enough lamports to cover the borrower_amount. We then transfer the collateral from the loan account to the borrower's account, and transfer the borrowed amount from the loan account to the borrower's account. Finally, we update the loan account with the borrower's public key and set is_initialized to false.

3. This program is still in development and i just copied few files in github repo for Solana Grizzlython Judges review.




 
    
     
       
       



       
      



