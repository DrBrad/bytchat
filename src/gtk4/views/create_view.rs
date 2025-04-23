use gtk4::{gdk, style_context_add_provider_for_display, Builder, CssProvider, Entry, Image, Label, ListBox, Paned, ScrolledWindow, Widget};
use gtk4::gio::{SimpleAction, SimpleActionGroup};
use gtk4::prelude::{ActionMapExt, Cast, EntryExt, ObjectExt, WidgetExt};
use crate::gtk4::views::inter::stackable::Stackable;

pub struct CreateView {
    pub root: gtk4::Box
}

impl CreateView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/com/bytchat/rust/res/ui/create_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/com/bytchat/rust/res/ui/create_view.css");
        style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in create_view.ui");

        let password: Entry = builder
            .object("password")
            .expect("Couldn't find 'password' in create_view.ui");

        let password_toggle: Image = builder
            .object("password_toggle")
            .expect("Couldn't find 'password_toggle' in create_view.ui");

        let actions = SimpleActionGroup::new();

        root.insert_action_group("password", Some(&actions));

        let action = SimpleAction::new("toggle", None);
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

        Self {
            root
        }
    }
}

impl Stackable for CreateView {

    fn get_name(&self) -> String {
        String::from("create_view")
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
