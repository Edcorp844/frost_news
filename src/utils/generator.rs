use chrono::{Datelike, Utc};
use rand::seq::{IndexedRandom, SliceRandom};

#[derive(Debug, Clone)]
pub struct Generator {}

impl Generator {
    pub fn generate_date() -> String {
        let mut years = vec!["2026", "2025", "2024"];
        let mut rng = rand::rng();
        years.shuffle(&mut rng);
        let year = years.choose(&mut rng);

        match *year.unwrap() {
            "2026" => {
                let month = 1;
                let mut days: Vec<i32> = (1..Utc::now().date_naive().day() as i32).collect();
                days.shuffle(&mut rng);
                let day = days.choose(&mut rng);
                format!(
                    "{}-{}-{}T16:39:57-08:00",
                    year.unwrap(),
                    Self::pad(month),
                    Self::pad(*day.unwrap())
                )
            }
            _ => {
                let mut months: Vec<i32> = (1..12).collect();
                months.shuffle(&mut rng);
                let month = months.choose(&mut rng);

                let mut days: Vec<i32> = match month.unwrap() {
                    2 => (1..28).collect(),
                    _ => (1..30).collect(),
                };
                days.shuffle(&mut rng);
                let day = months.choose(&mut rng);
                format!(
                    "{}-{}-{}T16:39:57-08:00",
                    year.unwrap(),
                    Self::pad(*month.unwrap()),
                    Self::pad(*day.unwrap())
                )
            }
        }
    }

    fn pad(number: i32) -> String {
        if number < 10 {
            return format!("0{}", number);
        }
        format!("{}", number)
    }
}
