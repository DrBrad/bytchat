use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;
use gtk4::{gdk, gio, style_context_add_provider_for_display, Application, ApplicationWindow, Builder, CssProvider, GestureClick, HeaderBar, Stack, StackPage, StyleContext, Widget};
use gtk4::prelude::{ApplicationWindowExt, BoxExt, Cast, GestureSingleExt, GtkApplicationExt, GtkWindowExt, ListModelExt, ObjectExt, StyleContextExt, WidgetExt};
use crate::database::sqlite::Database;
use crate::gtk4::actions::window_actions::register_window_actions;
use crate::gtk4::views::authentication_view::AuthenticationView;
use crate::gtk4::views::create_view::CreateView;
use crate::gtk4::views::inter::stackable::Stackable;
use crate::gtk4::views::main_view::MainView;

#[derive(Clone)]
pub struct MainWindow {
    pub window: ApplicationWindow,
    //pub title_bar: TitleBar,
    pub stack: Stack,
    //pub notifications: gtk4::Box,
    //pub bottom_bar: BottomBar,
    pub views: Rc<RefCell<HashMap<String, Box<dyn Stackable>>>>
}

impl MainWindow {

    pub fn new(app: &Application) -> Self {
        let builder = Builder::from_resource("/com/bytchat/rust/res/ui/window.ui");

        let window: ApplicationWindow = builder
            .object("main_window")
            .expect("Failed to get the 'main_window' from window.ui");

        window.set_application(Some(app));
        window.connect_destroy(|_| exit(0));
        //window.set_decorated(false);
        //window.set_border_width(1);

        let provider = CssProvider::new();
        provider.load_from_resource("/com/bytchat/rust/res/ui/window.css");

        style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        /*
        let (width, height) = window.default_size();

        let hints = Geometry::new(
            width,
            height,
            -1,
            -1,
            0,
            0,
            0,
            0,
            0.0,
            0.0,
            gdk::Gravity::NorthWest);
        window.set_geometry_hints(None::<&gtk::Widget>, Some(&hints), WindowHints::MIN_SIZE);
        */

        #[cfg(profile = "nightly")]
        window.style_context().add_class("nightly");

        #[cfg(profile = "release")]
        window.style_context().add_class("release");

        //window.set_icon_from_file("res/icons/ic_launcher.svg").expect("Failed to load icon");

        //let title_bar = TitleBar::new(&window);

        let root: gtk4::Box = builder
            .object("root")
            .expect("Failed to get the 'root' from window.ui");

        //window_content.add(&create_alertbar());



        /*
        //FULL SCREEN MAC PATCH...
        window.connect_fullscreened_notify({
            let root = root.clone();
            let title_bar = title_bar.clone();
            move |window| {
                if window.is_fullscreen() {
                    title_bar.root.remove(&title_bar.title_bar);
                    root.insert_child_after(&title_bar.title_bar, None::<&Widget>);
                    return;
                }

                root.remove(&title_bar.title_bar);
                title_bar.root.append(&title_bar.title_bar);
            }
        });*/


        let stack: Stack = builder
            .object("stack")
            .expect("Failed to get the 'stack' from window.ui");

        let views: Rc<RefCell<HashMap<String, Box<dyn Stackable>>>> = Rc::new(RefCell::new(HashMap::new()));

        stack.connect_visible_child_name_notify({
            let views = views.clone();
            let mut previous = RefCell::new(String::new());
            move |stack| {
                let current = stack.visible_child_name().unwrap_or_default().to_string();

                if previous.borrow().is_empty() {
                    *previous.borrow_mut() = current;
                    return;
                }

                views.borrow().get(&*previous.borrow()).unwrap().on_pause();

                if views.borrow().contains_key(&current) {
                    views.borrow().get(&current).unwrap().on_resume();
                }

                *previous.borrow_mut() = current;
            }
        });

        //let notifications: gtk4::Box = builder
        //    .object("notifications")
        //    .expect("Failed to get the 'notifications' from window.ui");

        //let bottom_bar = BottomBar::new();
        //root.append(&bottom_bar.root);

        window.set_focusable(true);
        window.set_can_focus(true);
        window.set_receives_default(true);

        window.set_show_menubar(true);


        window.show();

        let _self = Self {
            window,
            //title_bar,
            stack,
            //notifications,
            //bottom_bar,
            views
        };


        match Database::open_existing("app.db") {
            Ok(db) => {
                println!("Successfully connected to the database");
                _self.add_view(Box::new(MainView::new()));
            }
            Err(_) => {
                println!("Failed to connect to the database");
                //CREATE RSA KEY, ASK NAME ETC...
                //OR TRY USING RSA-KEY LOGIN...
                _self.window.set_show_menubar(false);
                _self.add_view(Box::new(CreateView::new()));
            }
        }

        //_self.add_view(Box::new(MainView::new()));

        register_window_actions(&_self);
        //register_stack_actions(&_self);

        _self
    }

    pub fn add_view(&self, view: Box<dyn Stackable>) {
        let name = view.get_name();

        match self.stack.child_by_name(&name) {
            Some(child) => {
                //self.title_bar.back.style_context().add_class("active");
                //self.title_bar.next.style_context().remove_class("active");

                let pages = self.stack.pages();
                for i in (0..pages.n_items()).rev() {
                    let page = pages.item(i).expect("Failed to get page")
                        .downcast::<StackPage>()
                        .expect("Item is not a StackPage");

                    let eq = child.eq(&page.child());
                    let name = page.name().unwrap().to_string();
                    self.views.borrow().get(&name).unwrap().on_destroy();
                    self.stack.remove(&page.child());
                    self.views.borrow_mut().remove(&name);

                    if eq {
                        break;
                    }
                }
            }
            None => {
                if let Some(current) = self.stack.visible_child() {
                    let pages = self.stack.pages();
                    for i in (0..pages.n_items()).rev() {
                        let page = pages.item(i).expect("Failed to get page")
                            .downcast::<StackPage>()
                            .expect("Item is not a StackPage");

                        if current.eq(&page.child()) {
                            //self.title_bar.back.style_context().add_class("active");
                            //self.title_bar.next.style_context().remove_class("active");
                            break;
                        }

                        let name = page.name().unwrap().to_string();
                        self.views.borrow().get(&name).unwrap().on_destroy();
                        self.stack.remove(&page.child());
                        self.views.borrow_mut().remove(&name);
                    }
                }
            }
        }

        self.stack.add_named(view.get_root(), Some(&name));
        self.stack.set_visible_child_name(&name);
        view.on_create();
        self.views.borrow_mut().insert(name, view);
    }

    /*
    pub fn notify(&self, _type: NotificationTypes, title: &str, description: &str) {
        self.notifications.append(&NotificationView::new(_type, title, description).root);
    }
    */
}
