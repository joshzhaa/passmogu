use passmogu::{
    encrypt::{decrypt, derive_key, encrypt},
    generate,
    secret::Secret,
    vault::{Field, Vault},
};

#[test]
#[allow(clippy::vec_init_then_push)]
fn basic_usage() {
    // Set the master password
    let master_password = Secret::new((*b"hunter2").into());
    let salt = b"arbitrary salt, just don't repeat it";
    let master_key = derive_key(master_password.expose(), salt);
    drop(master_password);

    let mut vault = Vault::new();
    let websites: [&[u8]; 4] = [
        b"your-bank.tld",
        b"the-irs.tld",
        b"the-tenth-airline-website-you-sign-up-for-to-get-points.tld",
        b"social-media-website.tld",
    ];
    for site in websites {
        // generate a password
        let password = generate::rand_base62(40).unwrap();
        // User enters plaintext login form data
        let mut plaintext_form: Vec<Field> = Vec::new();
        plaintext_form.push(Field {
            prompt: Secret::new((*b"Username").into()),
            answer: Secret::new((*b"AzureDiamond").into()),
        });
        plaintext_form.push(Field {
            prompt: Secret::new((*b"Password").into()),
            answer: password,
        });
        plaintext_form.push(Field {
            prompt: Secret::new((*b"Credit Card Number").into()),
            answer: Secret::new((*b"5555555555555555").into()),
        });
        plaintext_form.push(Field {
            prompt: Secret::new((*b"Social Security Number").into()),
            answer: Secret::new((*b"5555555555").into()),
        });
        plaintext_form.push(Field {
            prompt: Secret::new((*b"What's your mother's maiden name?").into()),
            answer: Secret::new((*b"Your mom!").into()),
        });

        // Encrypt form data
        let mut encrypted_form: Vec<Field> = Vec::new();
        for form in plaintext_form {
            let prompt = encrypt(form.prompt, master_key.expose()).unwrap();
            let answer = encrypt(form.answer, master_key.expose()).unwrap();

            encrypted_form.push(Field { prompt, answer });
        }
        let form_name = Secret::new(site.into());

        // Save form into vault
        vault.insert(
            encrypt(form_name, master_key.expose()).unwrap().expose(),
            encrypted_form.into_boxed_slice(),
        );
    }

    for form_name in vault.form_names() {
        let name = decrypt(Secret::new(form_name.into()), master_key.expose()).unwrap();
        assert!(websites.contains(&name.expose()));
        println!("\nform: {}", str::from_utf8(name.expose()).unwrap());
        let form = &vault[form_name];
        for field in form {
            let prompt = decrypt(field.prompt.clone(), master_key.expose()).unwrap();
            let answer = decrypt(field.answer.clone(), master_key.expose()).unwrap();
            assert_ne!(prompt, answer);
            println!(
                "{} {}",
                str::from_utf8(prompt.expose()).unwrap(),
                str::from_utf8(answer.expose()).unwrap()
            );
        }
    }

    // TODO: implement serialization
    // TODO: implement checking for the actual contents of a vault
}
