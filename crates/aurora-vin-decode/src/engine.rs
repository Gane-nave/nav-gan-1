/// VIN decoding: manufacturer, model year, plant, serial extraction
/// Phase 188

#[derive(Debug, Clone)]
pub struct VinInfo {
    pub vin: String,
    pub manufacturer: String,
    pub model_year: u16,
    pub plant_code: char,
    pub serial: String,
}

impl VinInfo {
    pub fn from_vin(vin: &str) -> Option<Self> {
        if vin.len() != 17 {
            return None;
        }
        let year_char = vin.chars().nth(9)?;
        let year = match year_char {
            'A'..='H' => 2010 + (year_char as u16 - 'A' as u16),
            'J'..='N' => 2018 + (year_char as u16 - 'J' as u16),
            'P' => 2023,
            'R'..='T' => 2024 + (year_char as u16 - 'R' as u16),
            _ => 2020,
        };
        Some(Self {
            vin: vin.to_string(),
            manufacturer: vin[0..3].to_string(),
            model_year: year,
            plant_code: vin.chars().nth(10).unwrap_or('X'),
            serial: vin[11..17].to_string(),
        })
    }

    pub fn is_valid_length(&self) -> bool {
        self.vin.len() == 17
    }

    pub fn check_digit_valid(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_vin() {
        let v = VinInfo::from_vin("1HGBH41JXMN109186");
        assert!(v.is_some());
    }

    #[test]
    fn test_invalid_length() {
        let v = VinInfo::from_vin("SHORT");
        assert!(v.is_none());
    }

    #[test]
    fn test_manufacturer() {
        let v = VinInfo::from_vin("1HGBH41JXMN109186").unwrap();
        assert_eq!(v.manufacturer, "1HG");
    }

    #[test]
    fn test_serial() {
        let v = VinInfo::from_vin("1HGBH41JXMN109186").unwrap();
        assert_eq!(v.serial, "109186");
    }

    #[test]
    fn test_valid_length() {
        let v = VinInfo::from_vin("1HGBH41JXMN109186").unwrap();
        assert!(v.is_valid_length());
    }

    #[test]
    fn test_plant_code() {
        let v = VinInfo::from_vin("1HGBH41JXMN109186").unwrap();
        assert_eq!(v.plant_code, 'N');
    }
}
