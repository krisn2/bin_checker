use reqwest::blocking::Client;
use serde_json::Value;
use std::io;
fn bin_info(bin: &str) {
    let url = format!("https://lookup.binlist.net/{}", bin);
    let client = Client::new();

    match client.get(&url).send() {
        Ok(response) => {
            if response.status().is_success() {
                match response.text() {
                    Ok(bin_info) => {
                        match serde_json::from_str::<Value>(&bin_info) {
                            Ok(bin_info) => {
                                println!("BIN: {}", bin);
                                println!("Length: {}", bin_info["number"]["length"]);
                                println!("Brand: {}", bin_info["scheme"]);
                                println!("Card Type: {}", bin_info["type"]);
                                println!("Country: {}", bin_info["country"]["name"]);
                                println!("Bank Name: {}", bin_info["bank"]["name"]);
                                println!("Bank URL: {}", bin_info["bank"]["url"]);
                                println!("Bank Phone: {}", bin_info["bank"]["phone"]);
                                println!("Bank Address: {}", bin_info["bank"]["address"]);
                                println!();
                            }
                            Err(e) => {
                                println!("Failed to parse JSON: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to read response text: {}", e);
                    }
                }
            } else {
                println!("Invalid BIN: {}", response.status());
            }
        }
        Err(e) => {
            println!("Failed to send request: {}", e);
        }
    }
}

fn mass_bin_info(bins: Vec<&str>) {
    for bin in bins {
        bin_info(bin);
    }
}

fn main() {
    loop {
        println!("Enter 1 for BIN lookup, 2 for mass BIN lookup, or Q to quit:");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        let input = user_input.trim();

        // Here we also use match expressions for better readability and for fast runtime

        if input.eq_ignore_ascii_case("Q") {
            break;
        } else if input == "1" {
            println!("Enter the BIN:");
            let mut bin = String::new();
            io::stdin().read_line(&mut bin).unwrap();
            bin_info(bin.trim());
        } else if input == "2" {
            println!("Enter BINs separated by commas:");
            let mut bins_input = String::new();
            io::stdin().read_line(&mut bins_input).unwrap();
            let bins: Vec<&str> = bins_input.trim().split(',').map(|s| s.trim()).collect();
            mass_bin_info(bins);
        } else {
            println!("Invalid input");
        }
    }
}
