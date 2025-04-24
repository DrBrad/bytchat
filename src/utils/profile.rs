use std::fs::{create_dir_all, File};
use std::{env, fs, io};
use std::io::Write;
use std::path::{Path, PathBuf};
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use openssl::symm::Cipher;

pub struct Profile {
    name: Option<String>,
    avatar: Option<PathBuf>,
    rsa: Rsa<Private>,
    password: String
}

impl Profile {

    pub fn new(password: &str) -> io::Result<Self> {
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

        Ok(Self {
            name: None,
            avatar: None,
            rsa,
            password: password.to_string()
        })
    }

    pub fn load(password: &str) -> io::Result<Self> {
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

        Ok(Self {
            name: None,
            avatar: None,
            rsa,
            password: password.to_string()
        })
    }

    pub fn exists() -> bool {
        let mut path = if let Ok(sudo_user) = env::var("SUDO_USER") {
            Path::new(&format!("/home/{}/", sudo_user)).to_path_buf()
        } else {
            env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/"))
        };
        path.push(".bytchat");
        path.push("keypair.pem");

        path.exists()
    }

    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn get_avatar(&self) -> Option<&PathBuf> {
        self.avatar.as_ref()
    }

    pub fn set_avatar(&mut self, avatar: Option<PathBuf>) {
        self.avatar = avatar;
    }
}
