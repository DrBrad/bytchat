use gtk4::{Builder, Label, ListBox};
use crate::gtk4::views::group_list_item::GroupListItem;
use crate::gtk4::views::message_list_item::MessageListItem;
use crate::gtk4::widgets::round_image::RoundImage;

pub struct MessagesView {
    pub root: gtk4::Box,
    pub icon: RoundImage,
    pub name: Label,
    pub messages_list: ListBox
}

impl MessagesView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/com/bytchat/rust/res/ui/messages_view.ui");
        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in messages_view.ui");

        let icon: RoundImage = builder
            .object("icon")
            .expect("Couldn't find 'icon' in messages_view.ui");
        icon.set_from_file(Some("/home/brad/Pictures/raw1.png"));

        let name: Label = builder
            .object("name")
            .expect("Couldn't find 'name' in messages_view.ui");
        name.set_label("Edward");

        let messages_list: ListBox = builder
            .object("messages_list")
            .expect("Couldn't find 'messages_list' in messages_view.ui");

        for i in 0..20 {
            let message = MessageListItem::new();
            messages_list.append(&message.root);
        }

        Self {
            root,
            icon,
            name,
            messages_list
        }
    }
}
