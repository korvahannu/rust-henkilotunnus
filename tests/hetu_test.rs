#[cfg(test)]
mod tests {
    use rust_henkilotunnus::henkilotunnus::*;

    #[test]
    fn test_valid_henkilotunnus() -> Result<(), ()> {
        let henkilotunnukset = vec![
            "010181-900C",
            "010100A963H",
            "311299-984Y",
            "040822A9456",
            "090921-987A",
            "020682+9287",
            "010100a963H",
            "010100A963h",
        ];

        for hetu in henkilotunnukset.iter() {
            let result = check_hetu(hetu.to_string());
            if result == Err(()) {
                return Err(());
            }
        }

        Ok(())
    }

    #[test]
    fn test_invalid_henkilotunnus() -> Result<(), ()> {
        let henkilotunnukset = vec![
            "010181-900",
            "010181-900HA",
            "010100A96H",
            "311299-94Y",
            "040822A456",
            "090921987A",
            "02068+9287",
            "01011-900C",
            "01000A963H",
            "31299-984Y",
            "00822A9456",
            "90921-987A",
            "000181-900C",
            "320100A963H",
            "311399-984Y",
            "040022A9456",
            "090921r987A",
            "010181z900C",
            "010100ö963H",
            "311299f984Y",
        ];

        for hetu in henkilotunnukset.iter() {
            let result = check_hetu(hetu.to_string());
            if result == Ok(()) {
                return Err(());
            }
        }

        Ok(())
    }

    fn check_hetu(hetu: String) -> Result<(), ()> {
        let hetu = Henkilotunnus::from(hetu.to_string());

        match hetu {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
