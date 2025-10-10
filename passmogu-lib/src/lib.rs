use std::collections::HashMap;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Form {
    pub site: String,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub prompt: String, // e.g. "username", "password", "What's your mother's maiden name?"
    pub answer: String,
}

pub struct Vault {
    pub forms: HashMap<String, Form>,
}

impl Vault {
    pub fn serialize() -> String {
        String::new()
    }
    pub fn deserialize() -> Vault {
        Vault { forms: HashMap::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

