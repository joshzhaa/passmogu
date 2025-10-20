use passmogu::{encrypt, vault};

#[test]
fn basic_usage() {
    let master_password = b"hunter2";
    let salt = b"arbitrary salt that isn't the same for everything";
    let master_key = encrypt::derive_key(master_password, salt);
    // don't need master_password past this point

    let vault = vault::Vault::new();
    let form = vault::Form::from(vec![]);
}
