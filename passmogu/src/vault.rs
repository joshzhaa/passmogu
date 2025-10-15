use std::collections::HashMap;
use std::ops::Index;

/// A field is a pair of prompt and answer e.g. ("password", "hunter2")
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Field {
    /// e.g. "username", "password", "What's your mother's maiden name?"
    prompt: Box<str>,
    /// value to populate into field
    answer: Box<str>,
}

/// A form is nothing more than a collection of fields to populate.
/// Any list of fields consitutes a valid form, so we simply define a type alias here.
type Form = Box<[Field]>;

/// Vault maps form_name -> form and mostly mirrors a subset of HashMap's API.
/// It's serializable to and from plaintext strings in a tsv format using dump and load.
/// The format is "form_name\tprompt1\tanswer1\tprompt2\tanswer2\n". The empty Vault is "" (not "\n").
/// Because of the tsv format, "\t" is disallowed in all fields. You probably didn't want it anyway.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Vault(HashMap<Box<str>, Form>);

impl Vault {
    /// Serializes vault into plaintext.
    pub fn dump(&self) -> Box<str> {
        // Could also parse twice to allocate the right size, then to populate, but it's easier this way.
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
        table.into_boxed_str()
    }

    /// Deserializes data from plaintext into Vault. Can only fail if string is malformed.
    pub fn load(data: &str) -> Option<Self> {
        let mut vault = Self(HashMap::new());
        for row in data.split('\n') {
            let mut i = row.split('\t');
            // expects name\tprompt\tanswer\tprompt\tanswer...
            let name = i.next()?; // each row must have a form name as the first token
            let mut form = Vec::new();
            while let Some(prompt) = i.next() {
                let answer = i.next()?; // each prompt must be paired with an answer
                form.push(Field {
                    prompt: prompt.into(),
                    answer: answer.into(),
                });
            }
            if name.is_empty() {
                continue; // permit empty rows but don't add "" as a key to the map
            }
            vault.insert(name.into(), form.into_boxed_slice());
        }
        Some(vault)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns Some &Form if form identified by name is present, None otherwise.
    pub fn get(&self, name: &str) -> Option<&Form> {
        self.0.get(name)
    }

    /// Returns names of forms currently stored in Vault
    pub fn form_names(&self) -> impl Iterator<Item = &str> {
        Keys(self.0.keys())
    }

    /// Writes or overwrites Vault\[name\]. The burden is on the caller to construct a Form.
    /// Returns None when no key was overwritten. Returns Some when a key was overwritten.
    pub fn insert(&mut self, name: Box<str>, form: Form) -> Option<Form> {
        self.0.insert(name, form)
    }

    /// Deletes a form in the Vault.
    /// Returns value which was removed, None if key wasn't in Vault.
    pub fn remove(&mut self, name: &str) -> Option<Form> {
        self.0.remove(name)
    }
}

impl Index<&str> for Vault {
    type Output = Form;

    fn index(&self, key: &str) -> &Self::Output {
        &self.0[key]
    }
}

/// vault::Keys Iterator adapts hash_map::Iter by auto deref-ing the Box into &str
pub struct Keys<'a>(std::collections::hash_map::Keys<'a, Box<str>, Form>);

impl<'a> Iterator for Keys<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(ptr) => Some(&**ptr),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_empty_vault() {
        let empty = "";
        let vault = Vault::load(empty).unwrap();
        assert!(vault.is_empty());
        assert_eq!(*vault.dump(), *empty);

        let tabs = "\t\t\t\t\n";
        let vault = Vault::load(tabs).unwrap();
        println!("{:?}", vault);
        assert!(vault.is_empty());
        assert_eq!(*vault.dump(), *empty);

        let newlines = "\n\n\n\n\n";
        let vault = Vault::load(newlines).unwrap();
        assert!(vault.is_empty());
        assert_eq!(*vault.dump(), *empty);

        let devault = Vault::default();
        assert!(vault.is_empty());
        assert_eq!(*devault.dump(), *empty);
    }

    #[test]
    fn serialize_basic_vault() {
        let serialized = "irc\tusername\tAzureDiamond\tpassword\thunter2\tWho's your best friend?\tCthon98\nother website dot com\tusername\tCthon98\tpassword\t*********\tWho's your best friend?\tAzureDiamond\n";
        let vault = Vault::load(serialized).unwrap();

        let mut names = vault.form_names();
        for _ in 0..vault.len() {
            let name = names.next().unwrap();
            assert!(name == "irc" || name == "other website dot com");
        }
        assert_eq!(names.next(), None);

        let first_form = &vault["irc"];
        assert_eq!(*first_form[0].prompt, *"username");
        assert_eq!(*first_form[0].answer, *"AzureDiamond");
        assert_eq!(*first_form[1].prompt, *"password");
        assert_eq!(*first_form[1].answer, *"hunter2");
        assert_eq!(*first_form[2].prompt, *"Who's your best friend?");
        assert_eq!(*first_form[2].answer, *"Cthon98");

        let second_form = &vault["other website dot com"];
        assert_eq!(*second_form[0].prompt, *"username");
        assert_eq!(*second_form[0].answer, *"Cthon98");
        assert_eq!(*second_form[1].prompt, *"password");
        assert_eq!(*second_form[1].answer, *"*********");
        assert_eq!(*second_form[2].prompt, *"Who's your best friend?");
        assert_eq!(*second_form[2].answer, *"AzureDiamond");

        // best friends
        assert_eq!(first_form[0].answer, second_form[2].answer);
        assert_eq!(first_form[2].answer, second_form[0].answer);

        assert_eq!(Vault::load(&vault.dump()), Vault::load(serialized));
    }

    #[test]
    fn modify_vault() {
        let mut vault = Vault::default();
        assert!(vault.is_empty());

        let generic_username = Field {
            prompt: "username".into(),
            answer: "user1@example.test".into(),
        };

        let bad_password = Field {
            prompt: "password".into(),
            answer: "password1".into(),
        };
        vault.insert(
            "asdf".into(),
            [generic_username.clone(), bad_password.clone()].into(),
        );

        assert_eq!(vault.len(), 1);
        assert_eq!(vault.get("form name that wasn't inserted"), None);

        let form = vault.get("asdf").unwrap();
        for field in form {
            assert!(field.prompt == generic_username.prompt || field.prompt == bad_password.prompt);
            assert!(field.answer == generic_username.answer || field.answer == bad_password.answer);
        }
        // the order of form fields is significant, it should be maintained
        assert_eq!(form[0], generic_username);
        assert_eq!(form[1], bad_password);

        vault.remove("asdf");
        assert!(vault.is_empty());
    }
}
