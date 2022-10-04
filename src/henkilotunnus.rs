const HENKILOTUNNUS_LENGTH: usize = 11;

/// The `Henkilotunnus` type
/// henkilotunnus: Full henkilotunnus as a String
/// year: Year as a two-digit String, 00..99
/// month: Month as a two-digit String 01..12
/// day_of_month: Day of month 0..31
/// century: Can be X, - or A. X stands for 1800-century, - for 1900-century and A for the 2000-century
/// checksum: Used to validate henkilotunnus
/// id: Used to validate henkilotunnus and to check gender
/// gender: Gender of the person in question f/m
#[derive(Debug)]
pub struct Henkilotunnus {
    pub henkilotunnus: String,
    pub year: String,
    pub month: String,
    pub day_of_month: String,
    pub century: char,
    pub checksum: char,
    pub id: String,
    pub gender: char,
    _private: ()
}

impl Henkilotunnus {
    /// Constructs a new `Henkilotunnus`.
    /// 
    /// Returns Err(&str) if the given string representing the henkilotunnus is invalid
    /// Returns Ok(Henkilotunnus) if the given string representing the henkilotunnus is valid
    pub fn from(mut henkilotunnus: String) -> Result<Henkilotunnus, &'static str> {
        if henkilotunnus.len() != HENKILOTUNNUS_LENGTH {
            return Err("Provided string was too short to be a valid henkilotunnus");
        }

        henkilotunnus = henkilotunnus.to_uppercase();

        let day_of_month;

        match substring(&henkilotunnus, 0, 2).parse::<u8>() {
            Ok(x) => {
                if x < 1 || x > 31 {
                    return Err("Invalid day of month (under 1 or over 31)");
                }

                if x < 10 {
                    day_of_month = format!("0{}", x.to_string());
                } else {
                    day_of_month = x.to_string()
                }
            }
            Err(_) => {
                return Err("Unable to parse day of month");
            }
        }

        let month;

        match substring(&henkilotunnus, 2, 2).parse::<u8>() {
            Ok(x) => {
                if x < 1 || x > 12 {
                    return Err("Invalid month (under 1 or over 31)");
                }

                if x < 10 {
                    month = format!("0{}", x.to_string());
                } else {
                    month = x.to_string()
                }
            }
            Err(_) => {
                return Err("Unable to parse month");
            }
        }

        let year;

        match substring(&henkilotunnus, 4, 2).parse::<u8>() {
            Ok(x) => {
                if x < 10 {
                    year = format!("0{}", x.to_string());
                } else {
                    year = x.to_string()
                }
            }
            Err(_) => {
                return Err("Unable to parse year");
            }
        }

        let century;
        match henkilotunnus.chars().nth(6) {
            Some(x) => {
                century = x;
            }
            None => {
                return Err("Unable to parse century");
            }
        }

        if century != '+' && century != '-' && century != 'A' {
            return Err("Invalid century");
        }

        let checksum;

        match henkilotunnus.chars().nth(10) {
            Some(x) => {
                checksum = x;
            }
            None => {
                return Err("Unable to parse checksum");
            }
        }

        let id = substring(&henkilotunnus, 7, 3);

        let id_number;

        match id.parse::<u16>() {
            Ok(x) => {

                if x < 1  {
                    return Err("Unable to parse id");
                }

                id_number = x;
            },
            Err(_) => {
                return Err("Unable to parse id");
            }
        }

        let gender;
        match id_number & 1 {
            0 => {
                gender = 'f';
            },
            1 => {
                gender = 'm';
            },
            _ => {
                return Err("Unable to figure out gender");
            }
        }

        let calculated_checksum = (id.parse::<usize>().unwrap() + year.parse::<usize>().unwrap() * 1000 + month.parse::<usize>().unwrap() * 100000 + day_of_month.parse::<usize>().unwrap() * 10000000) % 31;
        let mut checksum_chars = "0123456789ABCDEFHJKLMNPRSTUVWXY".chars();
        if checksum_chars.nth(calculated_checksum).unwrap() != checksum {
            return Err("Checksum and id mismatch");
        }

        Ok(Henkilotunnus {
            henkilotunnus: henkilotunnus,
            year,
            month,
            day_of_month,
            century,
            checksum,
            id,
            gender,
            _private: ()
        })
    }

    pub fn to_string(&self) -> String {
        format!("Henkilotunnus: {}
Year: {}
Month: {}
Day of month: {}
Century: {}
Checksum: {}
Id: {}
Gender: {}",
            &self.henkilotunnus,
            &self.year,
            &self.month,
            &self.day_of_month,
            &self.century,
            &self.checksum,
            &self.id,
            &self.gender,
        )
    }

    /// Returns the full year of struct Henkilotunnus as type usize
    pub fn full_year_as_usize(&self) -> usize {
        let mut output: usize = 0;

        output += &self.year.parse::<usize>().unwrap();

        match &self.century {
            '+' => output += 1800,
            '-' => output += 1900,
            'A' => output += 2000,
            _ => {}
        }

        output
    }
}

fn substring(string_to_parse: &String, start: usize, how_many_to_take: usize) -> String {
    string_to_parse
        .chars()
        .skip(start)
        .take(how_many_to_take)
        .collect::<String>()
}