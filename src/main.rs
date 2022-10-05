use std::io::{self};

use rust_henkilotunnus::henkilotunnus::*;

fn main() {
    println!("Hello, there! Please input a henkilotunnus to test:");
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match Henkilotunnus::from(input.trim_end().to_string()) {
                Ok(hetu) => {
                    // Given henkilotunnus is valid
                    println!("{}", hetu.to_string());
                }
                Err(message) => {
                    // Given henkilotunnus is invalid
                    println!("{}", message);
                }
            }
        }
        Err(e) => {
            println!("Oops! {}", e)
        }
    }
}
