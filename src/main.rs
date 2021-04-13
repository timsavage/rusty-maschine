use hidapi::HidApi;

use crate::gui::ui::Surface;
use colour::Colour;
use controller::Controller;
use devices::get_device;
use events::{Event, EventContext, EventHandler, EventTask};
use gui::display::Canvas;
use gui::ui::{ListPanel, TabPanel, TextPanel};

mod colour;
mod controller;
mod devices;
mod error;
mod events;
mod gui;
mod pad;

fn setup_ui(height: usize, width: usize) -> Surface<TabPanel> {
    // let logo = MonochromeCanvas::from_buffer(128, 64, &logo::LOGO);

    let mut surface: Surface<TabPanel> = Surface::new(height, width);

    let mut text_panel1 = TextPanel::new();
    text_panel1.set_text("This\nis some\ntext\n\nHello\nWorld\n\nThis is a new\nworld\nof\nstuff");

    let mut text_panel2 = TextPanel::new();
    text_panel2.set_text("The quick brown fox\njumps over the lazy\ndog.");

    let mut list_panel: ListPanel<u8> = ListPanel::new();
    list_panel.add_item(0, "Item A");
    list_panel.add_item(1, "Item B");
    list_panel.add_item(2, "Item C");
    list_panel.add_item(3, "Item D");
    list_panel.add_item(4, "Item E");
    list_panel.add_item(5, "Item F");
    list_panel.add_item(6, "Item G");
    list_panel.add_item(7, "Item H");
    list_panel.add_item(8, "Item I");
    list_panel.add_item(9, "Item J");

    let mut panel = TabPanel::new();
    panel.add_tab(0, "FIRST", Box::new(text_panel1));
    panel.add_tab(1, "SECOND", Box::new(text_panel2));
    panel.add_tab(2, "THIRD", Box::new(list_panel));
    surface.set_child(panel);

    surface
}

fn main() {
    let hid_api = HidApi::new().unwrap();
    let mut ctlr = get_device(&hid_api).unwrap();
    let mut surface = setup_ui(ctlr.display.height(), ctlr.display.width());
    let mut rainbow = pad::Rainbow::new();

    loop {
        // Paint the surface
        surface.paint(&mut ctlr.display);

        let mut context = EventContext::new();

        // Allow controller to do work and update any events
        ctlr.tick(&mut context).unwrap();
        rainbow.render(&mut ctlr);

        // Handle any generated events
        while !context.events.is_empty() {
            let event = context.events.pop_front().unwrap();

            surface.handle(&event);
            rainbow.handle(&event);
        }
    }
}
