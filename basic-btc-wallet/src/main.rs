use base58::ToBase58;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use clap::{Parser, Subcommand};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::SecretKey;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser)]
#[command(version, about = "Minimal Bitcoin Wallet CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GenerateSeed {
        #[arg(long, help = "Specify a custom file instead of default (wallet.seed)")]
        from: Option<String>,
    },

    GeneratePrivate {
        #[arg(
            long,
            help = "Specify a custom seed file instead of default (wallet.seed)"
        )]
        from: Option<String>,
    },

    GeneratePublic {
        #[arg(
            long,
            help = "Specify a custom private key file instead of default (wallet.private)"
        )]
        from: Option<String>,
    },

    GenerateAddress {
        #[arg(
            long,
            help = "Specify a custom public key file instead of default (wallet.public)"
        )]
        from: Option<String>,
    },
}

fn read_file_or_error(file_path: &str) -> String {
    match fs::read_to_string(file_path) {
        Ok(content) => content.trim().to_string(),
        Err(_) => {
            eprintln!(
                "Error: Expected file '{}' not found. Use --from to specify another file.",
                file_path
            );
            std::process::exit(1);
        }
    }
}

fn save_to_file(filename: &str, content: &str) {
    let path = Path::new(filename);

    if path.exists() {
        print!("File '{}' already exists. Overwrite? (yes/no): ", filename);
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        if answer.trim().to_lowercase() != "yes" {
            println!("Aborted.");
            return;
        }
    }

    fs::write(filename, content).expect("Failed to write to file");
    println!("Saved to: {}", filename);
}

fn generate_mnemonic() -> Mnemonic {
    Mnemonic::new(MnemonicType::Words24, Language::English)
}

fn mnemonic_to_seed(mnemonic: &Mnemonic) -> Vec<u8> {
    Seed::new(mnemonic, "").as_bytes().to_vec()
}

fn seed_to_private_key(seed: &[u8]) -> SecretKey {
    let bytes = &seed[..32];
    let private_key = SecretKey::from_bytes(bytes.into()).unwrap();
    private_key
}

fn private_to_public(private_key: &SecretKey) -> k256::PublicKey {
    private_key.public_key()
}

fn public_to_address(public_key: &k256::PublicKey) -> String {
    let binding = public_key.to_encoded_point(true);
    let public_key_bytes = binding.as_bytes();

    let sha256_hash = Sha256::digest(public_key_bytes);
    let ripemd160_hash = Ripemd160::digest(sha256_hash);

    let mut address_bytes = vec![0x00];
    address_bytes.extend(&ripemd160_hash);

    let checksum = Sha256::digest(&Sha256::digest(&address_bytes));
    address_bytes.extend(&checksum[..4]);

    address_bytes.to_base58()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateSeed { from } => {
            let file_path = from.unwrap_or_else(|| "wallet.seed".to_string());
            let mnemonic = generate_mnemonic();
            println!("Seed Phrase: {}", mnemonic);
            save_to_file(&file_path, &mnemonic.to_string());
        }
        Commands::GeneratePrivate { from } => {
            let file_path = from.unwrap_or_else(|| "wallet.seed".to_string());
            let seed_phrase = read_file_or_error(&file_path);
            let mnemonic = Mnemonic::from_phrase(&seed_phrase, Language::English)
                .expect("Invalid seed phrase!");
            let seed = mnemonic_to_seed(&mnemonic);
            let private_key = seed_to_private_key(&seed);
            let private_key_hex = hex::encode(private_key.to_bytes());
            println!("Private Key: {}", private_key_hex);
            save_to_file("wallet.private", &private_key_hex);
        }
        Commands::GeneratePublic { from } => {
            let file_path = from.unwrap_or_else(|| "wallet.private".to_string());
            let private_key_str = read_file_or_error(&file_path);
            let private_key_bytes =
                hex::decode(private_key_str).expect("Invalid private key format!");
            let private_key = SecretKey::from_bytes(private_key_bytes.as_slice().into())
                .expect("Invalid private key!");
            let public_key = private_to_public(&private_key);
            let public_key_hex = hex::encode(public_key.to_encoded_point(true).as_bytes());
            println!("Public Key: {}", public_key_hex);
            save_to_file("wallet.public", &public_key_hex);
        }
        Commands::GenerateAddress { from } => {
            let file_path = from.unwrap_or_else(|| "wallet.public".to_string());
            let public_key_str = read_file_or_error(&file_path);
            let public_key_bytes = hex::decode(public_key_str).expect("Invalid public key format!");
            let public_key =
                k256::PublicKey::from_sec1_bytes(&public_key_bytes).expect("Invalid public key!");
            let address = public_to_address(&public_key);
            println!("Bitcoin Address: {}", address);
            save_to_file("wallet.address", &address);
        }
    }
}
