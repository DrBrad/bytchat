use gtk4::{gdk, style_context_add_provider_for_display, Builder, CssProvider, Label, ListBox, Paned, ScrolledWindow, Widget};
use gtk4::prelude::Cast;
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
