use gtk4::{Builder, Label, ListBoxRow};

pub struct MessageListItem {
    pub root: ListBoxRow,
    pub message: Label
}

impl MessageListItem {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/com/bytchat/rust/res/ui/message_list_item.ui");
        let root: ListBoxRow = builder
            .object("root")
            .expect("Couldn't find 'root' in message_list_item.ui");

        let message: Label = builder
            .object("message")
            .expect("Couldn't find 'message' in message_list_item.ui");
        message.set_label("Whats up bro?");

        Self {
            root,
            message
        }
    }
}
