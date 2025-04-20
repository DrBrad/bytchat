use gtk4::{Builder, Label};
use crate::gtk4::widgets::round_image::RoundImage;

pub struct MessagesView {
    pub root: gtk4::Box,
    pub icon: RoundImage,
    pub name: Label
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

        Self {
            root,
            icon,
            name
        }
    }
}
