use passmogu::{
    encrypt::derive_key,
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

    // User saves login form data
    let mut form: Vec<Field> = Vec::new();
    form.push(Field {
        prompt: Secret::new((*b"Username").into()),
        answer: Secret::new((*b"AzureDiamond").into()),
    });
    form.push(Field {
        prompt: Secret::new((*b"Password").into()),
        answer: Secret::new((*b"hunter2").into()),
    });
    form.push(Field {
        prompt: Secret::new((*b"Credit Card Number").into()),
        answer: Secret::new((*b"5555555555555555").into()),
    });
    form.push(Field {
        prompt: Secret::new((*b"Social Security Number").into()),
        answer: Secret::new((*b"5555555555").into()),
    });
    form.push(Field {
        prompt: Secret::new((*b"What's your mother's maiden name?").into()),
        answer: Secret::new((*b"Your mom!").into()),
    });

    // Save first vault
    let mut vault = Vault::new();
    vault.insert(b"your-bank", form.clone().into_boxed_slice());
    vault.insert(b"the-irs", form.clone().into_boxed_slice());
    vault.insert(
        b"your-social-media-website-1",
        form.clone().into_boxed_slice(),
    );
    vault.insert(
        b"your-social-media-website-2",
        form.clone().into_boxed_slice(),
    );
    vault.insert(
        b"tenth-airline-website-you-have.points-for",
        form.clone().into_boxed_slice(),
    );
    vault.insert(
        b"tenth-hotel-website-you-have.points-for",
        form.clone().into_boxed_slice(),
    );
    // don't actually dump plaintext passwords
    println!("{}", str::from_utf8(&vault.dump()).unwrap());
}
