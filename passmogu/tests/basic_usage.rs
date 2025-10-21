// use passmogu::{encrypt, safe_string::SafeString, vault};

#[test]
fn basic_usage() {
    // let master_password: SafeString = SafeString::new((*b"hunter2").into());
    // let salt = b"arbitrary salt that isn't the same for everything";
    // let master_key = encrypt::derive_key(&master_password, salt);
    // drop(master_password);

    // let vault = vault::Vault::new();
    // let form = vault::Form::from(vec![]);
    // TODO: API for constructing a form, probably involve allocating a MAX_LEN amount of space
    // Then shrinking to fit when stored in vault
}
