use gtk4::{gdk, style_context_add_provider_for_display, Builder, CssProvider, Label, ListBox, Paned, ScrolledWindow, Widget};
use gtk4::prelude::Cast;
use crate::gtk4::views::group_list_item::GroupListItem;
use crate::gtk4::views::groups_view::GroupsView;
use crate::gtk4::views::inter::stackable::Stackable;
use crate::gtk4::views::messages_view::MessagesView;
use crate::gtk4::widgets::round_image::RoundImage;

pub struct MainView {
    pub root: gtk4::Box,
    pub groups_view: GroupsView,
    pub messages_view: MessagesView
}

impl MainView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/com/bytchat/rust/res/ui/main_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/com/bytchat/rust/res/ui/main_view.css");
        style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in main_view.ui");

        let activity_pane: Paned = builder
            .object("activity_pane")
            .expect("Couldn't find 'activity_pane' in main_view.ui");

        let groups_view = GroupsView::new();
        activity_pane.set_start_child(Some(&groups_view.root));

        let messages_view = MessagesView::new();
        activity_pane.set_end_child(Some(&messages_view.root));

        Self {
            root,
            groups_view,
            messages_view
        }
    }
}

impl Stackable for MainView {

    fn get_name(&self) -> String {
        String::from("main_view")
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
