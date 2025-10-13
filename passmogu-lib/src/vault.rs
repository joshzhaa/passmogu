use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug, PartialEq, Eq)]
pub struct Field {
    /// e.g. "username", "password", "What's your mother's maiden name?"
    pub prompt: String,
    /// value to populate into field
    pub answer: String,
}

/// A form is simply a collection of fields to populate.
/// All vectors of fields consitute a form, so we simply define a type alias here.
type Form = Vec<Field>;

/// HashMap mapping form_name -> form, serializable to and from plaintext strings in a tsv format
/// (which obviously disallows including \t in any fields).
/// The format is "form_name\tprompt1\tanswer1\tprompt2\tanswer2\n". The empty Vault is "" (not "\n").
#[derive(Debug, PartialEq, Eq)]
pub struct Vault(pub HashMap<String, Form>);

impl Vault {
    /// serialize to String
    pub fn dump(&self) -> String {
        let mut table = String::new();
        for (name, form) in &self.0 {
            table.push_str(name);
            for field in form.iter() {
                table.push('\t');
                table.push_str(&field.prompt);
                table.push('\t');
                table.push_str(&field.answer);
            }
            table.push('\n');
        }
        table
    }

    /// deserialize from &str
    pub fn load(data: &str) -> Option<Vault> {
        let mut vault = Vault(HashMap::new());
        for row in data.split('\n') {
            if row.is_empty() {
                continue; // permit empty rows but don't add "" as a key to the map
            }
            let mut i = row.split('\t');
            // expects name\tprompt\tanswer\tprompt\tanswer...
            let name = i.next()?.to_string(); // each row must have a form name as the first token
            let mut form = Form::new();
            while let Some(prompt) = i.next() {
                let answer = i.next()?; // each prompt must be paired with an answer
                form.push(Field {
                    prompt: prompt.to_string(),
                    answer: answer.to_string(),
                });
            }
            vault.0.insert(name, form);
        }
        Some(vault)
    }
}

impl Index<&str> for Vault {
    type Output = Form;

    fn index(&self, key: &str) -> &Self::Output {
        &self.0[key]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vault() {
        let serialized = "";
        let vault = Vault::load(serialized).unwrap();
        println!("{vault:?}");
        assert_eq!(vault.0.len(), 0);
        assert_eq!(vault.dump(), serialized);
    }

    #[test]
    fn basic_vault() {
        let serialized = "irc\tusername\tAzureDiamond\tpassword\thunter2\tWhat was the name of your best friend in elementary school?\tCthon98\nother website dot com\tusername\tCthon98\tpassword\t*********\tWhat was the name of your best friend in elementary school?\tAzureDiamond\n";
        let vault = Vault::load(serialized).unwrap();
        let first_form = &vault["irc"];
        assert_eq!(first_form[0].prompt, "username");
        assert_eq!(first_form[0].answer, "AzureDiamond");
        assert_eq!(first_form[1].prompt, "password");
        assert_eq!(first_form[1].answer, "hunter2");
        assert_eq!(
            first_form[2].prompt,
            "What was the name of your best friend in elementary school?"
        );
        assert_eq!(first_form[2].answer, "Cthon98");

        let second_form = &vault["other website dot com"];
        assert_eq!(second_form[0].prompt, "username");
        assert_eq!(second_form[0].answer, "Cthon98");
        assert_eq!(second_form[1].prompt, "password");
        assert_eq!(second_form[1].answer, "*********");
        assert_eq!(
            second_form[2].prompt,
            "What was the name of your best friend in elementary school?"
        );
        assert_eq!(second_form[2].answer, "AzureDiamond");

        assert_eq!(Vault::load(&vault.dump()), Vault::load(serialized));
    }
}
