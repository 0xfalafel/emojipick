use gtk::prelude::*;
use relm4::prelude::*;
use relm4::gtk::gdk::Display;

struct App;

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Simple app"),
            set_default_size: (200, 150),

            gtk::Button {
                set_margin_all: 15,
                // set_width_request: 150,
                // set_height_request: 150,
                set_label: "😁",
                connect_clicked => {
                    // Get the clipboard from Display
                    if let Some(display) = Display::default() {
                        let clipboard = display.clipboard();
                        clipboard.set_text("😁");
                    }
                    println!("Text copied to clipboard!");
                },
            },
        }
    }

    // Initialize the component.
    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App;

        // Insert the code generation of the view! macro here
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.helloworld");
    app.run::<App>(());
}
