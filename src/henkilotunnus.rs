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
    pub year: usize,
    pub month: usize,
    pub day_of_month: usize,
    pub century: char,
    pub checksum: char,
    pub id: String,
    pub gender: char,
    _private: (),
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

        match substring(&henkilotunnus, 0, 2).parse::<usize>() {
            Ok(x) => {
                if x < 1 || x > 31 {
                    return Err("Invalid day of month (under 1 or over 31)");
                }
                day_of_month = x;
            }
            Err(_) => {
                return Err("Unable to parse day of month");
            }
        }

        let month;

        match substring(&henkilotunnus, 2, 2).parse::<usize>() {
            Ok(x) => {
                if x < 1 || x > 12 {
                    return Err("Invalid month (under 1 or over 12)");
                }

                month = x;
            }
            Err(_) => {
                return Err("Unable to parse month");
            }
        }

        let century;
        match henkilotunnus.chars().nth(6) {
            Some(x) => {
                century = x;
            }
            _ => {
                return Err("Unable to parse century");
            }
        }

        if century != '+' && century != '-' && century != 'A' {
            return Err("Invalid century");
        }

        let two_digit_year;
        let mut year;

        match substring(&henkilotunnus, 4, 2).parse::<usize>() {
            Ok(x) => {
                year = x;
                two_digit_year = x;

                match century {
                    '+' => year += 1800,
                    '-' => year += 1900,
                    'A' => year += 2000,
                    _ => {
                        return Err("Unable to parse year");
                    }
                }
            }
            Err(_) => {
                return Err("Unable to parse year");
            }
        }

        let checksum;

        match henkilotunnus.chars().nth(10) {
            Some(x) => {
                checksum = x;
            }
            _ => {
                return Err("Unable to parse checksum");
            }
        }

        let id = substring(&henkilotunnus, 7, 3);

        let id_number;

        match id.parse::<u16>() {
            Ok(x) => {
                if x < 1 {
                    return Err("Unable to parse id");
                }

                id_number = x;
            }
            Err(_) => {
                return Err("Unable to parse id");
            }
        }

        let gender;
        match id_number & 1 {
            0 => {
                gender = 'f';
            }
            1 => {
                gender = 'm';
            }
            _ => {
                return Err("Unable to figure out gender");
            }
        }

        let calculated_checksum =
            (id.parse::<usize>().unwrap() + two_digit_year * 1000 + month * 100000 + day_of_month * 10000000)
                % 31;
        let mut checksum_chars = "0123456789ABCDEFHJKLMNPRSTUVWXY".chars();
        if checksum_chars.nth(calculated_checksum).unwrap() != checksum {
            return Err("Checksum and id mismatch");
        }
        
        Ok(Henkilotunnus {
            henkilotunnus,
            year,
            month,
            day_of_month,
            century,
            checksum,
            id,
            gender,
            _private: (),
        })
    }

    pub fn to_string(&self) -> String {
        format!(
            "Henkilotunnus: {}
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
}

fn substring(string_to_parse: &String, start: usize, how_many_to_take: usize) -> String {
    string_to_parse
        .chars()
        .skip(start)
        .take(how_many_to_take)
        .collect::<String>()
}
