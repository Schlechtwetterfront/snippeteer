extern crate chrono;
extern crate gtk;
extern crate gdk;

use std::sync::{Arc, Mutex};

use gtk::prelude::*;
use gtk::{
    Window, Clipboard, Builder
};

mod clipman;
use clipman::{ClipManager, ClipBuilder};

mod cmd;

mod gui;

fn main() {
    println!("revvin up");

    if gtk::init().is_err() {
        println!("Failed to initialize GTK");
        return;
    }

    // Build UI from glade source.
    let glade_src = include_str!("snippeteer.glade");
    let builder = Builder::new();

    if builder.add_from_string(glade_src).is_err() {
        println!("Failed to parse glade definition");
        return;
    }

    // Load custom styles.
    let css = gtk::CssProvider::new();
    if let Err(e) = css.load_from_path("styles.css") {
        println!("Failed to parse theme: {}", e);
        return;
    }

    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().unwrap(),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Get all the widgets we need by their glade ids.
    let window: Window = builder.get_object("window").unwrap();
    let clip_tree: gtk::TreeView = builder.get_object("tree_clips").unwrap();
    let named_clip_tree: gtk::TreeView = builder.get_object("tree_named_clips").unwrap();
    let input_box: gtk::Entry = builder.get_object("input_box").unwrap();
    let status_label: gtk::Label = builder.get_object("status_label").unwrap();

    let status = move |status: String| { status_label.set_text(&status); };

    // Set up clip managing.
    let clip_view = Arc::new(Mutex::new(gui::ClipView::with_tree(clip_tree)));
    let named_clip_view = Arc::new(Mutex::new(gui::ClipView::with_tree(named_clip_tree)));
    let clipman = Arc::new(Mutex::new(ClipManager::new()));

    let display = window.get_display().unwrap();
    let gclipboard = Clipboard::get_default(&display).unwrap();

    input_box.connect_key_release_event(move |widget, key| {
        use gdk::enums::key;

        match key.get_keyval() {
            key::Return => {
                match cmd::parse(widget.get_text().unwrap()) {
                    Ok(cmd::Command::Paste) => {
                        // Paste current clipboard into the manager.
                        let text = gclipboard.wait_for_text();

                        let mut locked = clipman.lock().unwrap();

                        let new_clip = locked
                            .new_clip()
                            .title(String::from("clip"))
                            .content(text.unwrap())
                            .build();

                        clip_view.lock().unwrap().add_clip(&new_clip);

                        status(format!("Wrote clipboard into {:?}", new_clip.key().unwrap()));

                        locked.add_clip(new_clip);
                    },
                    Ok(cmd::Command::PasteNamed(name)) => {
                        // Paste current clipboard into the manager with an explicit name.
                        let text = gclipboard.wait_for_text();

                        let mut locked = clipman.lock().unwrap();

                        let new_clip = ClipBuilder::new()
                            .title(String::from("clip"))
                            .key(name)
                            .content(text.unwrap())
                            .build();

                        named_clip_view.lock().unwrap().add_clip(&new_clip);

                        status(format!("Wrote clipboard into {:?}", new_clip.key().unwrap()));

                        locked.add_named_clip(new_clip);
                    },
                    Ok(cmd::Command::Copy(from)) => {
                        // Copy entry to clipboard.
                        let locked = clipman.lock().unwrap();

                        if let Some(ref clip) = locked.clip(from.clone()) {
                            gclipboard.set_text(&clip.content());
                            status(format!("Copied clipboard into {:?}", from));
                        } else {
                            status(format!("Could not find clip {}", from));
                        }
                    },
                    Ok(cmd::Command::Delete(id)) => {
                        status(format!("Deleted clip {:?}", id));
                    },
                    Ok(cmd::Command::Move(from, to)) => {
                        status(format!("Moved {:?} to {:?}", from, to));
                    },
                    Err(e) => println!("{:?}", e),
                }

                widget.set_text("");

                Inhibit(false)
            },
            _ => Inhibit(true)
        }
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
