//#[cfg(feature = "gtk3")]
//mod gtk3;
//#[cfg(feature = "gtk3")]
//use crate::gtk3::app::App;

#[cfg(feature = "gtk4")]
mod gtk4;
mod kad;

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
    run_p2p();

    let app = App::new();
    app.run();
}
