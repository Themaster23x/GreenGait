use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer, read_keypair_file},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;

/// Tranzacție simbolică: trimite 0.00001 SOL pentru a testa conexiunea și semnarea
pub async fn log_step_on_chain(
    user_pubkey: &str,
    steps: u64,
    _timestamp: i64,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let rpc_url = "https://api.devnet.solana.com";
    let rpc_client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Încarcă keypair-ul backendului din fișier
    let payer = read_keypair_file("certs/stepmint-validator.json")?;
    let recipient = Pubkey::from_str(user_pubkey)?;

    println!("[CHAIN] Creating transaction: {} steps → {}", steps, user_pubkey);

    // Construim o tranzacție simbolică: 0.00001 SOL (10_000 lamports)
    let instruction = system_instruction::transfer(
        &payer.pubkey(),
        &recipient,
        10_000,
    );

    let latest_blockhash = rpc_client.get_latest_blockhash().await?;
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    let sig = rpc_client.send_and_confirm_transaction(&tx).await?;
    println!("[CHAIN] Transaction confirmed ✅: {}", sig);

    Ok(sig)
}
