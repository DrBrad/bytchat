use gtk4::{Builder, Label, ListBox};
use crate::gtk4::views::group_list_item::GroupListItem;

pub struct GroupsView {
    pub root: gtk4::Box
}

impl GroupsView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/com/bytchat/rust/res/ui/groups_view.ui");
        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in groups_view.ui");

        let group_list: ListBox = builder
            .object("group_list")
            .expect("Couldn't find 'group_list' in groups_view.ui");

        for i in 0..20 {
            let group = GroupListItem::new();
            group_list.append(&group.root);
        }

        group_list.select_row(Some(&group_list.row_at_index(0).unwrap()));

        Self {
            root
        }
    }
}
