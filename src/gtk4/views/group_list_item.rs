use gtk4::{Builder, Image, Label, ListBoxRow};

pub struct GroupListItem {
    pub root: ListBoxRow,
    pub icon: Image,
    pub name: Label,
    pub message: Label
}

impl GroupListItem {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/com/bytchat/rust/res/ui/group_list_item.ui");
        let root: ListBoxRow = builder
            .object("root")
            .expect("Couldn't find 'root' in device_list_item.ui");

        let icon: Image = builder
            .object("icon")
            .expect("Couldn't find 'icon' in device_list_item.ui");

        let name: Label = builder
            .object("name")
            .expect("Couldn't find 'name' in device_list_item.ui");
        name.set_label("Edward");

        let message: Label = builder
            .object("message")
            .expect("Couldn't find 'message' in device_list_item.ui");
        message.set_label("Whats up bro?");

        Self {
            root,
            icon,
            name,
            message
        }
    }
}
