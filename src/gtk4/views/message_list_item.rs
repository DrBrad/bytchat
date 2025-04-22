use gtk4::{Builder, Label, ListBoxRow};
use gtk4::Align::{End, Start};
use gtk4::prelude::{StyleContextExt, WidgetExt};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MessageTypes {
    From,
    To
}

pub struct MessageListItem {
    pub root: ListBoxRow,
    pub _type: MessageTypes,
    pub message_container: gtk4::Box,
    pub message: Label,
    pub time: Label
}

impl MessageListItem {

    pub fn new(_type: MessageTypes) -> Self {
        let builder = Builder::from_resource("/com/bytchat/rust/res/ui/message_list_item.ui");

        let root: ListBoxRow = builder
            .object("root")
            .expect("Couldn't find 'root' in message_list_item.ui");

        let message_container: gtk4::Box = builder
            .object("message_container")
            .expect("Couldn't find 'message_container' in message_list_item.ui");

        match _type {
            MessageTypes::From => {
                message_container.style_context().add_class("from");
                message_container.set_halign(Start);
            },
            MessageTypes::To => {
                message_container.style_context().add_class("to");
                message_container.set_halign(End);
            }
        }

        let message: Label = builder
            .object("message")
            .expect("Couldn't find 'message' in message_list_item.ui");
        message.set_label("Whats up bro?");

        let time: Label = builder
            .object("time")
            .expect("Couldn't find 'time' in message_list_item.ui");
        time.set_label("10:05 AM");

        Self {
            root,
            _type,
            message_container,
            message,
            time
        }
    }
}
