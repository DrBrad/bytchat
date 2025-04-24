use gtk4::{glib, AboutDialog, License, Window};
use gtk4::gio::{AppInfo, AppLaunchContext, File, SimpleAction};
use gtk4::glib::{VariantDict, VariantTy};
use gtk4::prelude::{ActionMapExt, Cast, GtkWindowExt};
use crate::gtk4::windows::main_window::MainWindow;

pub fn register_window_actions(window: &MainWindow) {
    let action = SimpleAction::new("website", None);
    action.connect_activate(move |_, _| {
        AppInfo::launch_default_for_uri("https://bytchat.com", None::<&AppLaunchContext>).unwrap();
    });
    window.window.add_action(&action);

    let action = SimpleAction::new("show-about-dialog", None);
    action.connect_activate({
        let window = window.window.clone();
        move |_, _| {
            open_about_dialog(&window.upcast_ref());
        }
    });
    window.window.add_action(&action);
}

pub fn register_stack_actions(window: &MainWindow) {
    let action = SimpleAction::new("view", Some(&VariantTy::new("a{sv}").unwrap()));
    //let action = SimpleAction::new("open", Some(&glib::VariantTy::BYTE_STRING));//Some(&glib::VariantTy::ANY));
    action.connect_activate({
        let window = window.clone();
        move |_, param| {
            if let Some(param) = param {
                if let Some(dict) = param.get::<VariantDict>() {
                    if let Some(name) = dict.lookup::<String>("name").ok().unwrap() {
                        let view = match name.as_str() {
                            "main_view" => {
                                if let Some(_type) = dict.lookup::<String>("type").ok().unwrap() {
                                    match _type.as_str() {
                                        "device" => {
                                            //let device = Device::unserialize(&dict.lookup::<Vec<u8>>("device").ok().unwrap().unwrap());
                                            //Box::new(MainView::from_device(&window, &device))
                                        }
                                        "any" => {
                                            //Box::new(MainView::new(&window))
                                        }
                                        _ => unimplemented!()
                                    }
                                } else {
                                    unimplemented!()
                                }
                            }
                            _ => unimplemented!()
                        };

                        //window.add_view(view);
                    }
                }
            }
        }
    });
    window.window.add_action(&action);
}

pub fn open_about_dialog(window: &Window) {
    //let icon_paintable = Texture::from_resource("/com/bytchat/rust/res/icons/ic_launcher.svg");

    let dialog = AboutDialog::builder()
        .transient_for(window)
        .modal(true)
        .program_name("BytChat")
        .version(format!("{}-{}-{}", env!("PROFILE"), env!("CARGO_PKG_VERSION"), "gtk4").as_str())
        .authors(vec!["DrBrad"])
        .website_label("https://bytchat.com")
        .website("https://bytchat.com")
        .comments("")
        .copyright("Copyright (c) 2024 BytChat")
        .license_type(License::MitX11)
        //.logo(&icon_paintable)
        .build();

    dialog.present();
}
