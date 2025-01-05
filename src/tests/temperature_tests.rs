#[cfg(test)]
mod tests {
    use crate::temperature::{ITemperature, Temperature};
    use crate::cli_error::CliError;

    #[test]
    fn from_spec_positive_temp() {
        let pulses: Vec<usize> = vec![
            // humidity
            50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 70, 50, 26, 50, 70, 50, 26, 50, 26,
            50, 26, 50, 70, 50, 70, 50, 26, 50, 26, // temp
            50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 70, 50, 26, 50, 70, 50, 26,
            50, 70, 50, 70, 50, 70, 50, 70, 50, 70, // checksum
            50, 70, 50, 70, 50, 70, 50, 26, 50, 70, 50, 70, 50, 70, 50, 26,
        ];

        let sut = Temperature::new();
        let response = sut.decode(pulses);

        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(response.humidity == 65.2);
        assert!(response.temperature == 35.1);
    }

    #[test]
    fn from_spec_negative_temp() {
        let pulses: Vec<usize> = vec![
            // humidity
            50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 70, 50, 26, 50, 70, 50, 26, 50, 26,
            50, 26, 50, 70, 50, 70, 50, 26, 50, 26, // temp
            50, 70, 50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 70, 50, 70,
            50, 26, 50, 26, 50, 70, 50, 26, 50, 70, // checksum
            50, 26, 50, 70, 50, 70, 50, 70, 50, 26, 50, 26, 50, 70, 50, 70,
        ];

        let sut = Temperature::new();
        let response = sut.decode(pulses);

        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(response.humidity == 65.2);
        assert!(response.temperature == -10.1);
    }

    #[test]
    fn checksum() {
        let pulses: Vec<usize> = vec![
            // humidity
            50, 26, 50, 26, 50, 26, 50, 26, 50, 70, 50, 26, 50, 70, 50, 26, 50, 70, 50, 26, 50, 26,
            50, 26, 50, 70, 50, 70, 50, 26, 50, 26, // temp
            50, 70, 50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 26, 50, 70, 50, 70,
            50, 26, 50, 26, 50, 70, 50, 26, 50, 70, // checksum
            50, 26, 50, 70, 50, 70, 50, 70, 50, 26, 50, 26, 50, 70, 50, 70,
        ];

        let sut = Temperature::new();
        let response = sut.decode(pulses);

        match response {
            Err(CliError::Checksum) => assert!(true),
            _ => assert!(
                false,
                "Expected error: CliError::Checsum, but got a different result"
            ),
        }
    }
}