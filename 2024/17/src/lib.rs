// Ensure all relevant items are marked with `pub` keyword

const CHRISTMAS_EMOJIS: [char; 4] = ['🎅', '🤶', '🎄', '🎁'];

// Your Solution here ...
pub trait StringExt {
    fn anonymize_email(&self) -> Self;
}

impl StringExt for String {
    fn anonymize_email(&self) -> Self {
        match self.find("@") {
            Some(index) => self[..index].to_christmas_emoji() + &self[index..],
            None => self.to_christmas_emoji(),
        }
    }
}

pub trait StrExt {
    fn to_christmas_emoji(&self) -> String;
}

impl StrExt for str {
    fn to_christmas_emoji(&self) -> String {
        let mut result = String::new();
        for i in 0..self.chars().count() {
            let emoji_index = i % CHRISTMAS_EMOJIS.len();
            result.push(CHRISTMAS_EMOJIS[emoji_index]);
        }

        result
    }
}

pub fn main() {
    let emails = vec![
        "rudolph.therapysessions@northpole.com".to_string(),
        "elfhr.complaint@northpole.urgent".to_string(),
        "santas.rage.management@christmaschaos.noel".to_string(),
        "overtimepay.never@elfexploitation.workshop".to_string(),
        "mrs.claus.divorce.lawyer@northpole.legal".to_string(),
        "reindeer.workers.comp@antler.insurance".to_string(),
        "naughty.list.revenge@santasecrets.com".to_string(),
        "workshop.ptsd.support@elves.anonymous".to_string(),
        "performance.anxiety@santa.breakdown".to_string(),
        "existential.crisis@northpole.void".to_string(),
    ];

    for email in emails {
        let anonymized_email = email.anonymize_email(); // This is the API that Santa wants!
        println!("Original: {} -> Anonymized: {}", email, anonymized_email);
    }
}

#[cfg(test)]
mod test {
    use crate::StringExt;

    #[test]
    fn test_anonymize() {
        assert_eq!("santa".to_string().anonymize_email(), "🎅🤶🎄🎁🎅");
        assert_eq!("santa@north.pole".to_string().anonymize_email(), "🎅🤶🎄🎁🎅@north.pole");
    }
}
