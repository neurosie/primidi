use std::fmt::Display;

use chrono::{Datelike, NaiveDate};

pub struct RepublicanDate {
    year: u32,
    date: u32, // 0-indexed
}

const WEEKDAYS: [&str; 10] = [
    "primidi", "duodi", "tridi", "quartidi", "quintidi", "sextidi", "septidi", "octidi", "nonidi",
    "décadi",
];

const MONTHS: [&str; 12] = [
    "Vendémiaire",
    "Brumaire",
    "Frimaire",
    "Nivôse",
    "Pluviôse",
    "Ventôse",
    "Germinal",
    "Floréal",
    "Prairial",
    "Messidor",
    "Thermidor",
    "Fructidor",
];

const CELEBRATIONS: [&str; 6] = [
    "celebration of virtue",
    "celebration of talent",
    "celebration of labour",
    "celebration of convictions",
    "celebration of honors",
    "celebration of the Revolution",
];

const NUMERALS: [(&str, u32); 13] = [
    ("M", 1000),
    ("CM", 900),
    ("D", 500),
    ("CD", 400),
    ("C", 100),
    ("XC", 90),
    ("L", 50),
    ("XL", 40),
    ("X", 10),
    ("IX", 9),
    ("V", 5),
    ("IV", 4),
    ("I", 1),
];

impl Display for RepublicanDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.date < 360 {
            let month = self.date / 30;
            let day = self.date % 30 + 1;
            let weekday = WEEKDAYS[(self.date % 10) as usize];
            let month_name = MONTHS[month as usize];
            write!(f, "{weekday} {day} {month_name}")?;
        } else {
            write!(f, "{}", CELEBRATIONS[(self.date - 360) as usize])?;
        }
        let mut roman_year = String::new();
        let mut year = self.year;
        for (sym, val) in NUMERALS {
            while year >= val {
                roman_year += sym;
                year -= val;
            }
        }
        write!(f, ", Year {roman_year}")
    }
}

impl From<NaiveDate> for RepublicanDate {
    fn from(value: NaiveDate) -> Self {
        let new_years_year = if value < NaiveDate::from_ymd_opt(value.year(), 9, 22).unwrap() {
            value.year() - 1
        } else {
            value.year()
        };
        let year = new_years_year - 1791;
        let date = value
            .signed_duration_since(NaiveDate::from_ymd_opt(new_years_year, 9, 22).unwrap())
            .num_days();
        RepublicanDate {
            year: year as u32,
            date: date as u32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_into() {
        let republican_date: RepublicanDate = NaiveDate::from_ymd_opt(2023, 7, 8).unwrap().into();
        assert_eq!(republican_date.date, 289);
        assert_eq!(republican_date.year, 231);
    }
    #[test]
    fn test_display() {
        let republican_date: RepublicanDate = NaiveDate::from_ymd_opt(2023, 7, 8).unwrap().into();
        assert_eq!(
            format!("{republican_date}"),
            "décadi 20 Messidor, Year CCXXXI"
        )
    }
}
