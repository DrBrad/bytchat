mod kad;
mod database;

//#[cfg(feature = "gtk3")]
//mod gtk3;
//#[cfg(feature = "gtk3")]
//use crate::gtk3::app::App;

#[cfg(feature = "gtk4")]
mod gtk4;
mod utils;

use openssl::rsa::Rsa;
use crate::database::sqlite::Database;
#[cfg(feature = "gtk4")]
use crate::gtk4::app::App;
use crate::kad::run_p2p;
use crate::utils::key_utils::{create_profile_key, load_profile_key};
//export GTK_DEBUG=interactive

//glib-compile-resources res/gtk4/linux.gresources.xml --target=res/resources.gresources

/*
rustup install nightly
rustup override set nightly

sudo apt install libssl-dev
*/

fn main() {
    //let key = load_profile_key("asd").unwrap();

    let app = App::new();
    app.run();
}
