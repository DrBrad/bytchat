use gtk4::{gdk, style_context_add_provider_for_display, Builder, CssProvider, ListBox, ScrolledWindow, Widget};
use gtk4::prelude::Cast;
use crate::gtk4::views::group_list_item::GroupListItem;
use crate::gtk4::views::inter::stackable::Stackable;

pub struct MainView {
    pub root: gtk4::Box,
    pub list_group_layout: ScrolledWindow,
    pub list_messages_layout: ScrolledWindow
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

        let list_group_layout: ScrolledWindow = builder
            .object("list_group_layout")
            .expect("Couldn't find 'list_group_layout' in main_view.ui");


        let list_messages_layout: ScrolledWindow = builder
            .object("list_messages_layout")
            .expect("Couldn't find 'list_messages_layout' in main_view.ui");



        let group_list: ListBox = builder
            .object("group_list")
            .expect("Couldn't find 'group_list' in main_view.ui");

        for i in 0..20 {
            let group = GroupListItem::new();
            group_list.append(&group.root);
        }



        Self {
            root,
            list_group_layout,
            list_messages_layout
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
