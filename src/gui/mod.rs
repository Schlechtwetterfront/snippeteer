use gtk::prelude::*;
use gtk::{
    CellRendererText, ListStore, TreeView, TreeViewColumn, SelectionMode
};

use clipman::Clip;

fn add_column(tree: &TreeView, id: i32, title: &str) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    column.set_title(title);
    column.add_attribute(&cell, "text", id);
    column.set_spacing(20);
    match id {
        0 => {
            column.set_min_width(50);
            column.set_alignment(1.0);
            cell.set_property_xalign(1.0);
        },
        // This doesn't actually do anything to limit the column width because 
        // this is the last column. It however stops the tree from growing 
        // depending on the content length.
        _ => column.set_max_width(80),
    };
    tree.append_column(&column);
}

pub struct ClipView {
    pub tree: TreeView,
    pub model: ListStore,
}

impl ClipView {
    pub fn with_tree(tree: TreeView) -> Self {
        // add_column(&tree, 0, "ID");
        // add_column(&tree, 1, "Created");
        // add_column(&tree, 2, "Content");

        let selection = tree.get_selection();
        selection.set_mode(SelectionMode::None);

        tree.set_headers_visible(false);
        tree.set_enable_search(false);

        let model = ListStore::new(&[
            String::static_type(),
            String::static_type(),
            String::static_type(),
        ]);

        // tree.set_model(Some(&model));

        ClipView {
            tree: tree,
            model: model,
        }
    }

    pub fn add_clip(&mut self, clip: &Clip) {
        let mut formatted_string: String;
        if clip.content().chars().count() > 33 {
            formatted_string = clip.content().chars().take(33).collect();
            formatted_string = formatted_string.replace("\n", "\\n") + "...";
        } else {
            formatted_string = clip.content().replace("\n", "\\n");
        }
        self.model.insert_with_values(
            None,
            &[0, 1, 2],
            &[
                // This is horrible but needed (?) to convert from String to &str.
                &clip.key().unwrap(),
                &clip.created().format("%H:%M:%S").to_string(),
                // This is horrible but needed (?) to convert from String to &str.
                &formatted_string,
            ]
        );
        self.tree.set_model(Some(&self.model));
    }
}
