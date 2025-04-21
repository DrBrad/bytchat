use gtk4::{Builder, Label, ListBoxRow};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MessageTypes {
    From,
    To
}

pub struct MessageListItem {
    pub root: ListBoxRow,
    pub message_container: gtk4::Box,
    pub message: Label
}

impl MessageListItem {

    pub fn new(_type: MessageTypes) -> Self {
        let builder = match _type {
            MessageTypes::From => Builder::from_resource("/com/bytchat/rust/res/ui/from_message_list_item.ui"),
            MessageTypes::To => Builder::from_resource("/com/bytchat/rust/res/ui/to_message_list_item.ui")
        };
        //let builder = Builder::from_resource("/com/bytchat/rust/res/ui/from_message_list_item.ui");
        let root: ListBoxRow = builder
            .object("root")
            .expect("Couldn't find 'root' in from_message_list_item.ui");

        let message_container: gtk4::Box = builder
            .object("message_container")
            .expect("Couldn't find 'message_container' in from_message_list_item.ui");

        let message: Label = builder
            .object("message")
            .expect("Couldn't find 'message' in from_message_list_item.ui");
        message.set_label("Whats up bro?");

        Self {
            root,
            message_container,
            message
        }
    }
}
