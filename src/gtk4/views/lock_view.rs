use gtk4::{gdk, style_context_add_provider_for_display, Builder, CssProvider, Entry, Image, Label, ListBox, Paned, ScrolledWindow, Widget};
use gtk4::gio::{SimpleAction, SimpleActionGroup};
use gtk4::prelude::{ActionGroupExt, ActionMapExt, Cast, EditableExt, EntryExt, ObjectExt, WidgetExt};
use crate::gtk4::views::inter::stackable::Stackable;
use crate::gtk4::windows::main_window::MainWindow;
use crate::utils::profile::Profile;

pub struct LockView {
    pub root: gtk4::Box
}

impl LockView {

    pub fn new(window: &MainWindow) -> Self {
        let builder = Builder::from_resource("/com/bytchat/rust/res/ui/lock_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/com/bytchat/rust/res/ui/lock_view.css");
        style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in lock_view.ui");

        let password: Entry = builder
            .object("password")
            .expect("Couldn't find 'password' in lock_view.ui");

        let password_toggle: Image = builder
            .object("password_toggle")
            .expect("Couldn't find 'password_toggle' in lock_view.ui");

        let actions = SimpleActionGroup::new();

        root.insert_action_group("lock", Some(&actions));

        let action = SimpleAction::new("password_toggle", None);
        action.connect_activate({
            let password = password.clone();
            move |_, _| {
                match password.property::<bool>("visibility") {
                    true => {
                        password.set_visibility(false);
                        password_toggle.set_resource(Some("/com/bytchat/rust/res/icons/ic_visible.svg"));
                    }
                    false => {
                        password.set_visibility(true);
                        password_toggle.set_resource(Some("/com/bytchat/rust/res/icons/ic_visible_off.svg"));
                    }
                }
            }
        });
        actions.add_action(&action);

        let action = SimpleAction::new("submit", None);
        action.connect_activate({
            let window = window.window.clone();
            let password = password.clone();
            move |_, _| {
                let password = password.text().to_string();
                println!("SUBMIT {}", password);

                match Profile::load(&password) {
                    Ok(profile) => {
                        ActionGroupExt::activate_action(&window, "view", None);//Some(&params));
                    }
                    Err(_) => {
                        println!("Invalid password");
                    }
                }
            }
        });
        actions.add_action(&action);

        Self {
            root
        }
    }
}

impl Stackable for LockView {

    fn get_name(&self) -> String {
        String::from("lock_view")
    }

    fn get_root(&self) -> &Widget {
        self.root.upcast_ref()
    }

    fn on_create(&self) {
    }

    fn on_resume(&self) {
    }

    fn on_pause(&self) {
    }

    fn on_destroy(&self) {
    }
}
