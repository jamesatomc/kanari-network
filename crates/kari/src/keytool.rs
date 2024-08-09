use std::io;
use colored::Colorize;
use std::process::exit;

use crate::blockchain::{BALANCES, load_blockchain};
use crate::blockchain_simulation;
use crate::wallet::{generate_karix_address, list_wallet_files, load_wallet, save_wallet, send_coins};

pub fn handle_keytool_command() -> Option<String> {
    // Collect command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if any arguments were provided
    if args.len() > 2 {
        // Collect command line arguments
        let command = &args[2];
        // Use string comparison in the match statement
        match command.as_str() {
            "generate" => { 
                println!("Enter mnemonic length (12 or 24):");
                let mut mnemonic_length_str = String::new();
                io::stdin().read_line(&mut mnemonic_length_str).unwrap();
                let mnemonic_length: usize = mnemonic_length_str.trim().parse().expect("Invalid input");

                let (private_key, public_address, seed_phrase) = generate_karix_address(mnemonic_length);
                println!("New address generated:");
                println!("Private Key: {}", private_key.green());
                println!("Public Address: {}", public_address.green());
                println!("Seed Phrase: {}", seed_phrase.green());

                save_wallet(&public_address, &private_key, &seed_phrase);

                return Some(public_address);
            },
            "balance" => { // String comparison for "balance"
                println!("Enter public address:");
                let mut public_address = String::new();
                io::stdin().read_line(&mut public_address).unwrap();
                let public_address = public_address.trim().to_string();

                load_blockchain();

                let balance = unsafe {
                    BALANCES.as_ref().unwrap().lock().unwrap().get(&public_address).cloned().unwrap_or(0)
                };

                println!("Balance for {}: {}", public_address.green(), balance.to_string().green());
                return None; // Return None to indicate no address to be used further
            },
            "wallet" => { // String comparison for "wallet"
                println!("Enter public address to load:");
                let mut public_address = String::new();
                io::stdin().read_line(&mut public_address).unwrap();
                let public_address = public_address.trim().to_string();

                if let Some(wallet_data) = load_wallet(&public_address) {
                    println!("Wallet loaded:");
                    println!("Address: {}", wallet_data["address"].as_str().unwrap().green());
                    println!("Private Key: {}", wallet_data["private_key"].as_str().unwrap().green());
                    println!("Seed Phrase: {}", wallet_data["seed_phrase"].as_str().unwrap().green());
                    return Some(public_address);
                } else {
                    println!("Wallet not found for address: {}", public_address.red());
                    return None; // Return None to indicate no address to be used further
                }
            },
            "send"  => { // String comparison for "send"
                println!("Enter sender's public address:");
                let mut sender_address = String::new();
                io::stdin().read_line(&mut sender_address).unwrap();
                let sender_address = sender_address.trim().to_string();

                println!("Enter receiver's public address:");
                let mut receiver_address = String::new();
                io::stdin().read_line(&mut receiver_address).unwrap();
                let receiver_address = receiver_address.trim().to_string();

                println!("Enter amount to send:");
                let mut amount_str = String::new();
                io::stdin().read_line(&mut amount_str).unwrap();
                let amount: u64 = amount_str.trim().parse().expect("Invalid input for amount");

                if let Some(transaction) = send_coins(sender_address, receiver_address, amount) {
                    // Send the transaction to the blockchain simulation thread
                    unsafe { // Access static variable within an unsafe block
                        if let Err(e) = blockchain_simulation::TRANSACTION_SENDER.as_ref().unwrap().send(transaction) {
                            println!("Failed to send transaction: {}", e);
                            return None; // Return None after handling the error
                        } else {
                            println!("Transaction added to pending transactions.");
                            return None; // Return None after successful sending
                        }
                    }
                } else {
                    println!("Transaction failed.");
                    return None; // Return None if send_coins fails
                }
            },
            "list" => { // String comparison for "list"}
                match list_wallet_files() {
                    Ok(wallets) => {
                        println!("Wallet files:");
                        for wallet in wallets {
                            println!("{}", wallet.green());
                        }
                    },
                    Err(e) => {
                        println!("Failed to list wallet files: {}", e);
                    }
                }
                return None;
            },
            _ => {
                println!("{}", "Invalid command".red());
                println!("Usage: kari keytool <command> [options]");
                println!("Commands:");
                println!("  {} - Generate new address", "generate".green());
                println!("  {} - Check balance", "balance".green());
                println!("  {} - Load existing wallet", "wallet".green());
                println!("  {} - Send coins", "send".green());
                println!("  {} - List wallet files", "list".green());
                exit(1); // Exit with an error code
        
            },
        }
    } else {
        // No command provided, print usage
        println!("Usage: kari keytool <command> [options]");
        println!("Commands:");
        println!("  {} - Generate new address", "generate".green());
        println!("  {} - Check balance", "balance".green());
        println!("  {} - Load existing wallet", "wallet".green());
        println!("  {} - Send coins", "send".green());
        println!("  {} - List wallet files", "list".green());
        exit(1); // Exit with an error code
    }
}