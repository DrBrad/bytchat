use std::cell::RefCell;
use std::path::Path;
use gtk4::{gio, glib, graphene, gsk, Buildable, Orientation, Picture, Snapshot, Widget};
use gtk4::gdk::Texture;
use gtk4::gio::File;
use gtk4::glib::property::PropertyGet;
use gtk4::graphene::Rect;
use gtk4::prelude::{Cast, DisplayExt, ObjectExt, PaintableExt, SnapshotExt, WidgetExt};
use gtk4::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassExt, WidgetImpl, WidgetImplExt};

const MIN_WIDTH: i32 = 20;
const MIN_HEIGHT: i32 = 20;

#[derive(Default)]
pub struct RoundImageImpl {
    picture: RefCell<Picture>
}

#[glib::object_subclass]
impl ObjectSubclass for RoundImageImpl {

    const NAME: &'static str = "RoundImage";
    type ParentType = Widget;
    type Type = RoundImage;

    fn class_init(class: &mut Self::Class) {
        class.set_css_name("roundimage");
    }
}

impl ObjectImpl for RoundImageImpl {}

impl WidgetImpl for RoundImageImpl {

    fn snapshot(&self, snapshot: &Snapshot) {
        let widget = self.obj();
        let width = widget.width() as f32;
        let height = widget.height() as f32;

        let cr = snapshot.append_cairo(&Rect::new(0.0, 0.0, width, height));

        let radius = f32::min(width, height) / 2.0;

        snapshot.push_rounded_clip(&gsk::RoundedRect::from_rect(
            Rect::new(0.0, 0.0, width, height),
            radius
        ));

        if let Some(paintable) = self.picture.borrow().paintable() {
            if let Some(texture) = paintable.downcast_ref::<Texture>() {
                snapshot.append_texture(texture, &Rect::new(0.0, 0.0, width, height));
            }
        }

        snapshot.pop();
    }

    fn measure(&self, orientation: Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        match orientation {
            Orientation::Horizontal => (MIN_WIDTH, MIN_WIDTH, -1, -1),
            Orientation::Vertical => (MIN_HEIGHT, MIN_HEIGHT, -1, -1),
            _ => unimplemented!()
        }
    }
}

glib::wrapper! {
    pub struct RoundImage(ObjectSubclass<RoundImageImpl>)
        @extends Picture, Widget, @implements Buildable;
}

impl RoundImage {

    pub fn new() -> Self {
        let _self = glib::Object::builder::<RoundImage>().build();
        _self
    }

    pub fn set_from_file(&self, path: Option<&str>) {
        if let Ok(texture) = Texture::from_file(&File::for_path(Path::new(path.unwrap()))) {
            self.imp().picture.borrow().set_paintable(Some(&texture));
        }
    }
}
