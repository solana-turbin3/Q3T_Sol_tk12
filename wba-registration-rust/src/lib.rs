use bs58;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[cfg(test)]
mod tests {
    use super::*;
    use bs58;
    use std::io::{self, BufRead};

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
        let wallet = read_wallet_from_file("./dev-wallet.json");
        println!("Your private key is:");
        let base58 = bs58::encode(wallet.0).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn airdrop() {
        // Add airdrop test implementation here
    }

    #[test]
    fn transfer_sol() {
        // Add transfer_sol test implementation here
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Wallet(Vec<u8>);

    fn read_wallet_from_file(file_path: &str) -> Wallet {
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let wallet: Wallet = serde_json::from_reader(reader).expect("Unable to parse JSON");
        wallet
    }

    fn read_base58_from_file(file_path: &str) -> String {
        let file = File::open(file_path).expect("Unable to open file");
        let reader = BufReader::new(file);
        let base58: String = serde_json::from_reader(reader).expect("Unable to parse JSON");
        base58
    }
}
