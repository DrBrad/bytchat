mod kad;
mod database;

//#[cfg(feature = "gtk3")]
//mod gtk3;
//#[cfg(feature = "gtk3")]
//use crate::gtk3::app::App;

#[cfg(feature = "gtk4")]
mod gtk4;

use openssl::rsa::Rsa;
use crate::database::sqlite::Database;
#[cfg(feature = "gtk4")]
use crate::gtk4::app::App;
use crate::kad::run_p2p;

//export GTK_DEBUG=interactive

//glib-compile-resources res/gtk4/linux.gresources.xml --target=res/resources.gresources

/*
rustup install nightly
rustup override set nightly
*/

fn main() {
    //run_p2p();

    /*
    let rsa = Rsa::generate(4096).expect("failed to generate key");

    // Export private key in PEM format
    let private_pem = rsa.private_key_to_pem().expect("failed to export private key");

    // Export public key in PEM format
    let public_pem = rsa.public_key_to_pem().expect("failed to export public key");

    println!("Private Key (PEM):\n{}", String::from_utf8_lossy(&private_pem));
    println!("Public Key (PEM):\n{}", String::from_utf8_lossy(&public_pem));
    */

    match Database::open_existing("app.db") {
        Ok(db) => {
            println!("Successfully connected to the database");
        }
        Err(_) => {
            println!("Failed to connect to the database");
            //CREATE RSA KEY, ASK NAME ETC...
            //OR TRY USING RSA-KEY LOGIN...
        }
    }


    let app = App::new();
    app.run();
}
