use gtk::{gdk, Builder, Container, CssProvider, Paned, Stack, StyleContext};
use gtk::glib::Cast;
use gtk::prelude::{BuilderExtManual, CssProviderExt, StackExt};
use crate::ui::application::OApplication;
use crate::ui::fragments::inter::fragment::Fragment;

pub struct DevicesFragment {
    app: OApplication,
    root: Option<gtk::Box>
}

impl DevicesFragment {

    pub fn new(app: OApplication) -> Self {
        Self {
            app,
            root: None
        }
    }
}

impl Fragment for DevicesFragment {

    fn get_name(&self) -> String {
        "devices_fragment".to_string()
    }

    fn get_title(&self) -> String {
        "DevicesFragment".to_string()
    }

    fn on_create(&mut self) -> &Container {
        let builder = Builder::from_file("res/ui/gtk3/devices-fragment.ui");

        let provider = CssProvider::new();
        provider.load_from_path("res/ui/gtk3/devices-fragment.css").expect("Failed to load CSS file.");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );


        self.root = Some(builder
            .object("devices_layout")
            .expect("Couldn't find 'devices_layout' in devices-fragment.ui"));

        &self.root.as_ref().unwrap().upcast_ref()
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
