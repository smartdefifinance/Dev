import { useState } from "react";
import { PublicKey } from "@solana/web3.js";
import { borrowLoan, createLoan, LendingBorrowingInstruction } from "./lendingBorrowing";

function App() {
  const [lenderAmount, setLenderAmount] = useState(0);
  const [borrowerAmount, setBorrowerAmount] = useState(0);
  const [collateralAmount, setCollateralAmount] = useState(0);
  const [lenderPubkey, setLenderPubkey] = useState("");
  const [borrowerPubkey, setBorrowerPubkey] = useState("");
  const [loanPubkey, setLoanPubkey] = useState("");

  const handleCreateLoan = async () => {
    const loanPubkey = await createLoan(lenderAmount, collateralAmount, new PublicKey(lenderPubkey));
    setLoanPubkey(loanPubkey.toString());
  };

  const handleBorrowLoan = async () => {
    await borrowLoan(borrowerAmount, new PublicKey(borrowerPubkey), new PublicKey(loanPubkey));
  };

  return (
    <div>
      <h1>Lending and Borrowing Protocol</h1>
      <h2>Create a Loan</h2>
      <label htmlFor="lenderAmount">Lender Amount:</label>
      <input id="lenderAmount" type="number" value={lenderAmount} onChange={(e) => setLenderAmount(Number(e.target.value))} />
      <br />
      <label htmlFor="collateralAmount">Collateral Amount:</label>
      <input id="collateralAmount" type="number" value={collateralAmount} onChange={(e) => setCollateralAmount(Number(e.target.value))} />
      <br />
      <label htmlFor="lenderPubkey">Lender Public Key:</label>
      <input id="lenderPubkey" type="text" value={lenderPubkey} onChange={(e) => setLenderPubkey(e.target.value)} />
      <br />
      <button onClick={handleCreateLoan}>Create Loan</button>
      <p>Loan Public Key: {loanPubkey}</p>
      <h2>Borrow a Loan</h2>
      <label htmlFor="borrowerAmount">Borrower Amount:</label>
      <input id="borrowerAmount" type="number" value={borrowerAmount} onChange={(e) => setBorrowerAmount(Number(e.target.value))} />
      <br />
      <label htmlFor="borrowerPubkey">Borrower Public Key:</label>
      <input id="borrowerPubkey" type="text" value={borrowerPubkey} onChange={(e) => setBorrowerPubkey(e.target.value)} />
      <br />
      <label htmlFor="loanPubkey">Loan Public Key:</label>
      <input id="loanPubkey" type="text" value={loanPubkey} onChange={(e) => setLoanPubkey(e.target.value)} />
      <br />
      <button onClick={handleBorrowLoan}>Borrow Loan</button>
    </div>
  );
}

export default App;
