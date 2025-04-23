use gtk4::{gdk, style_context_add_provider_for_display, Builder, CssProvider, Label, ListBox, Notebook, Paned, ScrolledWindow, Widget};
use gtk4::prelude::{Cast, WidgetExt};
use crate::gtk4::views::create_view::CreateView;
use crate::gtk4::views::group_list_item::GroupListItem;
use crate::gtk4::views::groups_view::GroupsView;
use crate::gtk4::views::inter::stackable::Stackable;
use crate::gtk4::views::messages_view::MessagesView;
use crate::gtk4::widgets::round_image::RoundImage;

pub struct AuthenticationView {
    pub root: gtk4::Box,
    pub tab_view: Notebook
}

impl AuthenticationView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/com/bytchat/rust/res/ui/authentication_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/com/bytchat/rust/res/ui/authentication_view.css");
        style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in authentication_view.ui");

        let tab_view: Notebook = builder
            .object("tab_view")
            .expect("Couldn't find 'tab_view' in authentication_view.ui");

        let view = CreateView::new();
        let label = Label::new(Some("Create Account"));
        label.set_hexpand(true);
        tab_view.append_page(&view.root, Some(&label));

        /*
        let view = CreateView::new();
        let label = Label::new(Some("Sign In"));
        label.set_hexpand(true);
        tab_view.append_page(&view.root, Some(&label));
        */

        Self {
            root,
            tab_view
        }
    }
}

impl Stackable for AuthenticationView {

    fn get_name(&self) -> String {
        String::from("authentication_view")
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
