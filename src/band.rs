use ethkey::{Brain, BrainPrefix, Generator};

const BRAIN_WORDS: usize = 12;

#[derive(Debug, PartialEq)]
pub struct Band {
    pub phrase:  String,
    pub secret:  String,
    pub public:  String,
    pub address: String,
}

impl Band {
    pub fn generate() -> Self {
        let mut brain = BrainPrefix::new(vec![0], usize::max_value(), BRAIN_WORDS);
        let keypair = brain.generate().unwrap().to_owned();

        Band {
            phrase:  brain.phrase().to_string(),
            secret:  keypair.secret().to_string(),
            public:  keypair.public().to_string(),
            address: keypair.address().to_string(),
        }
    }

    pub fn validate_phrase(phrase: &str) -> bool {
        match Brain::validate_phrase(phrase, BRAIN_WORDS) {
	    Ok(()) => true,
	    Err(_err) => false
        }
    }
}

impl From<String> for Band {
    fn from(phrase: String) -> Self {
        let band = Brain::new(phrase.to_string()).generate().unwrap();
        Band {
            phrase:  phrase,
            secret:  band.secret().to_string(),
            public:  band.public().to_string(),
            address: band.address().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Band;
    #[test]
    fn band_of_hourses() {
        let band = Band::generate();
        let test = Band::from(band.phrase.to_string());
        assert_eq!(band, test);
    }
}
