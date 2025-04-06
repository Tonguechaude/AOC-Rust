fn main() {
    println!("Hello, world!");
}

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
        // 🎁 Transform the CSV row into a Kid struct for Santa's list!
        // 🎅 Expected CSV: "Name,GoodDeeds,BadDeeds"
        //    Example: "Alice,3,1" -> name: "Alice", good_deeds: 3, bad_deeds: 1
        // 🎁 Your code here! 🎁
        let mut éléments = csv_row.split(',');
        let name = éléments.next().ok_or("Missing name")?.to_string();
        let good_deeds = éléments
            .next()
            .ok_or("Missing good deeds")?
            .parse::<u32>()
            .map_err(|_| "good_deeds Invalid")?;
        let bad_deeds = éléments
            .next()
            .ok_or("Missing bad_deeds")?
            .parse::<u32>()
            .map_err(|_| "bad_deeds Invalid")?;
        Ok(Self::new(name, good_deeds, bad_deeds))
    }

    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Self {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Self { name, niceness }
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);

        ratio >= 0.75
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_csv_parse() {
//         let csv_row = "Alice,3";
//         let kid = Kid::parse_row(csv_row).unwrap();
//     }
// }
