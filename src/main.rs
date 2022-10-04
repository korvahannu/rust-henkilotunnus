use rust_henkilotunnus::henkilotunnus::*;

fn main() {
    match Henkilotunnus::from(String::from("010181-900C")) {
        Ok(hetu) => {
            // Given henkilotunnus is valid
            println!("{}", hetu.to_string());
        },
        Err(message) => {
            // Given henkilotunnus is invalid
            println!("{}", message);
        }
    }
}
