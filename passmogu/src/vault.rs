use std::collections::HashMap;
use std::ops::Index;

/// A field is a pair of prompt and answer e.g. ("password", "hunter2")
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Field {
    /// e.g. "username", "password", "What's your mother's maiden name?"
    prompt: Box<[u8]>,
    /// value to populate into field
    answer: Box<[u8]>,
}

/// A form is nothing more than a collection of fields to populate.
/// Any list of fields consitutes a valid form, so we simply define a type alias here.
type Form = Box<[Field]>;

/// Vault maps form_name -> form and mostly mirrors a subset of HashMap's API.
/// It's serializable to and from tsv. The format is "form_name\tprompt1\tanswer1\tprompt2\tanswer2\n".
/// The empty Vault is "" (not "\n"). Because of the tsv format, "\t" is disallowed in all fields.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Vault(HashMap<Box<[u8]>, Form>);

impl Vault {
    /// Serializes vault into tsv.
    pub fn dump(&self) -> Box<[u8]> {
        // Could also parse twice to allocate the right size, then to populate, but it's easier this way.
        let mut table: Vec<u8> = Vec::new();
        for (name, form) in &self.0 {
            table.extend(name);
            for field in form.iter() {
                table.push(b'\t');
                table.extend(&field.prompt);
                table.push(b'\t');
                table.extend(&field.answer);
            }
            table.push(b'\n');
        }
        table.into_boxed_slice()
    }

    /// Deserializes data from tsv into Vault. Can only fail if string is malformed.
    pub fn load(data: &[u8]) -> Option<Self> {
        let mut vault = Self(HashMap::new());
        for row in data.split(|byte| *byte == b'\n') {
            let mut i = row.split(|byte| *byte == b'\t');
            // expects name\tprompt\tanswer\tprompt\tanswer...
            let name = i.next()?; // each row must have a form name as the first token
            let mut form = Vec::new();
            while let Some(prompt) = i.next() {
                let answer = i.next()?; // each prompt must be paired with an answer
                form.push(Field {
                    prompt: Box::from(prompt),
                    answer: Box::from(answer),
                });
            }
            if name.is_empty() {
                continue; // permit empty rows but don't add "" as a key to the map
            }
            vault.insert(Box::from(name), form.into_boxed_slice());
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
    pub fn get(&self, name: &[u8]) -> Option<&Form> {
        self.0.get(name)
    }

    /// Returns names of forms currently stored in Vault.
    pub fn form_names(&self) -> impl Iterator<Item = &[u8]> {
        Keys(self.0.keys())
    }

    /// Writes or overwrites Vault\[name\]. The burden is on the caller to construct a Form.
    /// Returns None when no key was overwritten. Returns Some when a key was overwritten.
    pub fn insert(&mut self, name: Box<[u8]>, form: Form) -> Option<Form> {
        self.0.insert(name, form)
    }

    /// Deletes a form in the Vault.
    /// Returns value which was removed, None if key wasn't in Vault.
    pub fn remove(&mut self, name: &[u8]) -> Option<Form> {
        self.0.remove(name)
    }
}

impl Index<&[u8]> for Vault {
    type Output = Form;

    fn index(&self, key: &[u8]) -> &Self::Output {
        &self.0[key]
    }
}

/// vault::Keys Iterator adapts hash_map::Iter by auto deref-ing the Box into &str
pub struct Keys<'a>(std::collections::hash_map::Keys<'a, Box<[u8]>, Form>);

impl<'a> Iterator for Keys<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(ptr) => Some(ptr),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_empty_vault() {
        let empty = b"";
        let vault = Vault::load(empty).unwrap();
        assert!(vault.is_empty());
        assert_eq!(*vault.dump(), *empty);

        let tabs = b"\t\t\t\t\n";
        let vault = Vault::load(tabs).unwrap();
        assert!(vault.is_empty());
        assert_eq!(*vault.dump(), *empty);

        let newlines = b"\n\n\n\n\n";
        let vault = Vault::load(newlines).unwrap();
        assert!(vault.is_empty());
        assert_eq!(*vault.dump(), *empty);

        let devault = Vault::default();
        assert!(vault.is_empty());
        assert_eq!(*devault.dump(), *empty);
    }

    #[test]
    fn serialize_basic_vault() {
        let serialized = b"irc\tusername\tAzureDiamond\tpassword\thunter2\tWho's your best friend?\tCthon98\nother website dot com\tusername\tCthon98\tpassword\t*********\tWho's your best friend?\tAzureDiamond\n";
        let vault = Vault::load(serialized).unwrap();

        let mut names = vault.form_names();
        for _ in 0..vault.len() {
            let name = names.next().unwrap();
            assert!(name == b"irc" || name == b"other website dot com");
        }
        assert_eq!(names.next(), None);

        let first_form = &vault[b"irc"];
        assert_eq!(*first_form[0].prompt, *b"username");
        assert_eq!(*first_form[0].answer, *b"AzureDiamond");
        assert_eq!(*first_form[1].prompt, *b"password");
        assert_eq!(*first_form[1].answer, *b"hunter2");
        assert_eq!(*first_form[2].prompt, *b"Who's your best friend?");
        assert_eq!(*first_form[2].answer, *b"Cthon98");

        let second_form = &vault[b"other website dot com"];
        assert_eq!(*second_form[0].prompt, *b"username");
        assert_eq!(*second_form[0].answer, *b"Cthon98");
        assert_eq!(*second_form[1].prompt, *b"password");
        assert_eq!(*second_form[1].answer, *b"*********");
        assert_eq!(*second_form[2].prompt, *b"Who's your best friend?");
        assert_eq!(*second_form[2].answer, *b"AzureDiamond");

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
            prompt: Box::from(*b"username"),
            answer: Box::from(*b"user1@example.test"),
        };

        let bad_password = Field {
            prompt: Box::from(*b"password"),
            answer: Box::from(*b"password1"),
        };
        vault.insert(
            Box::from(*b"asdf"),
            [generic_username.clone(), bad_password.clone()].into(),
        );

        assert_eq!(vault.len(), 1);
        assert_eq!(vault.get(b"form name that wasn't inserted"), None);

        let form = vault.get(b"asdf").unwrap();
        for field in form {
            assert!(field.prompt == generic_username.prompt || field.prompt == bad_password.prompt);
            assert!(field.answer == generic_username.answer || field.answer == bad_password.answer);
        }
        // the order of form fields is significant, it should be maintained
        assert_eq!(form[0], generic_username);
        assert_eq!(form[1], bad_password);

        vault.remove(b"asdf");
        assert!(vault.is_empty());
    }
}
