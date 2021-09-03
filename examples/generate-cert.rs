use rcgen::generate_simple_self_signed;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let subject_alt_names = vec!["localhost".to_string()];

    let cert = generate_simple_self_signed(subject_alt_names).unwrap();
    fs::write("cert.pem", cert.serialize_pem().unwrap())?;
    fs::write("key.pem", cert.serialize_private_key_pem())?;

    Ok(())
}
