use gtk4::Widget;
use crate::gtk4::views::inter::stackable::Stackable;

pub struct MainView {

}

impl MainView {

    pub fn new() -> Self {
        Self {

        }
    }
}

impl Stackable for MainView {

    fn get_name(&self) -> String {
        todo!()
    }

    fn get_root(&self) -> &Widget {
        todo!()
    }

    fn on_create(&self) {
        todo!()
    }

    fn on_resume(&self) {
        todo!()
    }

    fn on_pause(&self) {
        todo!()
    }

    fn on_destroy(&self) {
        todo!()
    }
}
