use gtk4::{Builder, Label, ListBox, ScrolledWindow};
use gtk4::prelude::{AdjustmentExt, WidgetExt};
use crate::gtk4::views::group_list_item::GroupListItem;
use crate::gtk4::views::message_list_item::{MessageListItem, MessageTypes};
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
        icon.set_from_file(Some("/home/brad/Pictures/raw.png"));

        let name: Label = builder
            .object("name")
            .expect("Couldn't find 'name' in messages_view.ui");
        name.set_label("Edward");

        let messages_list: ListBox = builder
            .object("messages_list")
            .expect("Couldn't find 'messages_list' in messages_view.ui");

        let mut messages: Vec<MessageListItem> = Vec::new();

        for i in 0..20 {
            if i % 4 == 0 {
                let message = MessageListItem::new(MessageTypes::To);
                messages_list.append(&message.root);

                if !messages.is_empty() {
                    if  messages.get(messages.len() - 1).unwrap()._type == MessageTypes::To {
                        message.message_container.add_css_class("previous");
                        messages.get(messages.len() - 1).unwrap().message_container.add_css_class("next");

                    } else {
                        messages.get(messages.len() - 1).unwrap().message_container.add_css_class("gap");
                    }
                }

                messages.push(message);
                continue;
            }

            let message = MessageListItem::new(MessageTypes::From);
            messages_list.append(&message.root);

            if !messages.is_empty() {
                if messages.get(messages.len() - 1).unwrap()._type == MessageTypes::From {
                    message.message_container.add_css_class("previous");
                    messages.get(messages.len() - 1).unwrap().message_container.add_css_class("next");

                } else {
                    messages.get(messages.len() - 1).unwrap().message_container.add_css_class("gap");
                }
            }

            messages.push(message);
        }



        let list_messages_layout: ScrolledWindow = builder
            .object("list_messages_layout")
            .expect("Couldn't find 'list_messages_layout' in messages_view.ui");

        //let adj = list_messages_layout.vadjustment();
        //adj.set_value(adj.upper() - adj.page_size());

        Self {
            root,
            icon,
            name,
            messages_list
        }
    }
}
