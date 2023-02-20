use solana_client::rpc_client::RpcClient;
use solana_program::program_pack::Pack;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::fs::File;
use std::io::Read;

fn main() {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());

    // Load the program binary file
    let mut file = File::open("path/to/program.so").unwrap();
    let mut program_data = Vec::new();
    file.read_to_end(&mut program_data).unwrap();

    // Create a new keypair for the program
    let program_keypair = Keypair::new();

    // Define the program's account
    let program_account = rpc_client
        .get_minimum_balance_for_rent_exemption(program_data.len())
        .unwrap();
    let mut transaction = Transaction::new_with_payer(
        &[solana_sdk::system_instruction::create_account(
            &program_keypair.pubkey(),
            &rpc_client.get_account(&program_keypair.pubkey()).unwrap().lamports,
            program_account,
            program_data.len() as u64,
            &solana_program::id(),
        )],
        Some(&rpc_client.get_recent_blockhash().unwrap()),
    );
    transaction.sign(&[&program_keypair], rpc_client.get_recent_blockhash().unwrap());
    rpc_client.send_and_confirm_transaction(&transaction).unwrap();

    // Load the program account
    let program_data_account = rpc_client
        .get_account(&program_keypair.pubkey())
        .unwrap();

    // Confirm that the program account contains the correct data
    let program_data_len = program_data.len();
    let mut program_data_account_data = vec![0; program_data_len];
    program_data_account_data.copy_from_slice(&program_data);
    assert_eq!(
        program_data_account.data[..program_data_len],
        program_data_account_data
    );
}
