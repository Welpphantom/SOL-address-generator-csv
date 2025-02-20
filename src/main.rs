use solana_sdk::signature::Keypair;
use solana_sdk::bs58;
use std::fs::File;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use csv::Writer;
use solana_sdk::signature::Signer;
fn main() {
    // Ask user for number of addresses to generate
    println!("Enter the number of Solana addresses to generate:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_keys: usize = input.trim().parse().expect("Please enter a valid number");

    // Generate timestamp for filename
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let filename = format!("sol-keypair-gen-{}.csv", timestamp);

    // Create CSV writer
    let mut wtr = Writer::from_writer(File::create(&filename).expect("Failed to create file"));
    wtr.write_record(["Private Key", "Public Key"]).expect("Failed to write header");

    // Generate keypairs and write to CSV
    for _ in 0..num_keys {
        let keypair = Keypair::new();
        let private_key = bs58::encode(keypair.to_bytes()).into_string();
        let public_key = keypair.pubkey().to_string();
        wtr.write_record([private_key, public_key]).expect("Failed to write record");
    }

    wtr.flush().expect("Failed to flush CSV writer");
    println!("Successfully generated {} Solana keypairs and saved to {}", num_keys, filename);
}