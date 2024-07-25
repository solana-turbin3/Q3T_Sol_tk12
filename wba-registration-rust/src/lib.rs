// public address of new wallet: J939BZY2XykKTTvNQYBxBLZperPSBrWuqM2WzMca9hwz

#[cfg(test)]
mod tests {

    use bs58;
    use serde::{Deserialize, Serialize};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
    };
    use std::fs::File;
    use std::io::BufReader;
    use std::str::FromStr;

    use solana_sdk::message::Message;

    const RPC_URL: &str = "https://api.devnet.solana.com";
    const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        let wallet_base58 = read_base58_from_file("dev-wallet-base58.json");
        println!("Your wallet file is:");
        let wallet = bs58::decode(wallet_base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        let wallet = read_wallet_from_file("dev-wallet.json").expect("Couldn't find wallet file");
        println!("Your private key is:");
        let base58 = bs58::encode(wallet.0).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn airdrop() -> Result<(), Box<dyn std::error::Error>> {
        let wallet = read_wallet_from_file("dev-wallet.json")?;
        let keypair = Keypair::from_bytes(&wallet.0)?;
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };

        Ok(())
    }

    #[test]
    fn transfer_sol() -> Result<(), Box<dyn std::error::Error>> {
        let wallet = read_wallet_from_file("dev-wallet.json")?;
        let keypair = Keypair::from_bytes(&wallet.0)?;
        let wba_wallet_public_key = "HaN2KEjyMxHsgCUBXjW3ahyqHD5dyuULd4tEPBbwZx4S";
        let to_pubkey = Pubkey::from_str(wba_wallet_public_key).unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Create the transfer instruction for 0.1 SOL
        let amount_in_lamports = LAMPORTS_PER_SOL / 10;

        // Sign the TX
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, amount_in_lamports)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );

        Ok(())
    }

    #[test]
    fn transfer_all_sol() -> Result<(), Box<dyn std::error::Error>> {
        let wallet = read_wallet_from_file("dev-wallet.json")?;
        let keypair = Keypair::from_bytes(&wallet.0)?;
        let wba_wallet_public_key = "HaN2KEjyMxHsgCUBXjW3ahyqHD5dyuULd4tEPBbwZx4S";
        let to_pubkey = Pubkey::from_str(wba_wallet_public_key).unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // get remaining balance of dev wallet
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        // create message from mock tx to get transaction fee
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        // Sign the TX
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );

        Ok(())
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Wallet(Vec<u8>);

    fn read_wallet_from_file(file_path: &str) -> Result<Wallet, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let wallet: Wallet = serde_json::from_reader(reader)?;
        Ok(wallet)
    }

    fn read_base58_from_file(file_path: &str) -> String {
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let base58: String = serde_json::from_reader(reader).expect("Unable to parse JSON");
        base58
    }
}
