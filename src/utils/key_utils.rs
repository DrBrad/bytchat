use std::fs::{create_dir_all, File};
use std::{env, fs, io};
use std::io::Write;
use std::path::{Path, PathBuf};
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use openssl::symm::Cipher;

pub fn create_profile_key(password: &str) -> io::Result<Rsa<Private>> {
    let rsa = Rsa::generate(4096)?;

    let mut path = if let Ok(sudo_user) = env::var("SUDO_USER") {
        Path::new(&format!("/home/{}/", sudo_user)).to_path_buf()
    } else {
        env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/"))
    };
    path.push(".bytchat");

    create_dir_all(&path)?;
    path.push("keypair.pem");

    let cipher = Cipher::aes_256_cbc();
    let private_pem = rsa.private_key_to_pem_passphrase(cipher, password.as_bytes())?;
    let public_pem = rsa.public_key_to_pem()?;

    let mut file = File::create(path)?;
    file.write_all(&private_pem)?;
    file.write_all(&public_pem)?;

    Ok(rsa)
}

pub fn load_profile_key(password: &str) -> io::Result<Rsa<Private>> {
    let mut path = if let Ok(sudo_user) = env::var("SUDO_USER") {
        Path::new(&format!("/home/{}/", sudo_user)).to_path_buf()
    } else {
        env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/"))
    };
    path.push(".bytchat");

    path.push("keypair.pem");

    let data = fs::read(&path)?;

    let rsa = Rsa::private_key_from_pem_passphrase(&data, password.as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(rsa)
}
