use gtk4::{gdk, style_context_add_provider_for_display, Application, CssProvider, Settings};
use gtk4::gio::{resources_register, ApplicationFlags, Resource};
use gtk4::glib::Bytes;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, ObjectExt, StaticType};
use crate::gtk4::widgets::overlay::Overlay;
use crate::gtk4::widgets::round_image::RoundImage;
use crate::gtk4::windows::main_window::MainWindow;

pub struct App {
    app: Application
}

impl App {

    pub fn new() -> Self {
        let app = Application::new(Some("com.bytchat.rust"), ApplicationFlags::HANDLES_OPEN);

        Self {
            app
        }
    }

    pub fn run(&self) {
        Overlay::static_type();
        RoundImage::static_type();

        self.app.connect_activate(move |app| {
            if let Some(settings) = Settings::default() {
                settings.set_property("gtk-application-prefer-dark-theme", &true);
            }

            let resource_data = include_bytes!("../../res/resources.gresources");

            let resource = Resource::from_data(&Bytes::from(resource_data)).unwrap();
            resources_register(&resource);


            let provider = CssProvider::new();
            provider.load_from_resource("/com/bytchat/rust/res/ui/theme.css");

            style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

            //provider.load_from_resource("/net/ethernaught/rust/res/ui/theme.css");

            /*
            #[cfg(target_os = "macos")]
            {
                let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/ethernaught_ui.xml");
                let model: gio::MenuModel = builder
                    .object("main_window_menu")
                    .expect("Couldn't find 'main_window_menu' in ethernaught_ui.xml");

                app.set_menubar(Some(&model));
            }
            */

            MainWindow::new(&app);

            //register_app_actions(&app);
        });

        self.app.run();
    }
}
