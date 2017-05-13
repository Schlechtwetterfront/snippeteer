use std::collections::HashMap;

mod clip;
pub use self::clip::{Clip, ClipBuilder};

pub struct ClipManager {
    clips: HashMap<String, Clip>,
    named_clips: HashMap<String, Clip>,
    current_id: usize,
}

impl ClipManager {
    pub fn new() -> Self {
        ClipManager {
            clips: HashMap::new(),
            named_clips: HashMap::new(),
            current_id: 1,
        }
    }

    pub fn new_clip(&mut self) -> ClipBuilder {
        let clip_id = self.current_id;
        self.current_id += 1;
        ClipBuilder {
            title: String::from(""),
            key: Some(clip_id.to_string()),
            content: String::from(""),
        }
    }

    pub fn add_clip(&mut self, mut clip: Clip) {
        if clip.key() == None {
            let clip_id = self.current_id;
            clip.set_key(clip_id.to_string());
            self.current_id += 1;
        }
        self.clips.insert(clip.key().unwrap(), clip);
    }

    pub fn add_named_clip(&mut self, clip: Clip) {
        self.named_clips.insert(clip.key().unwrap(), clip);
    }

    pub fn clip(&self, key: String) -> Option<&Clip> {
        self.clips.get(&key)
    }

    pub fn named_clip(&self, key: String) -> Option<&Clip> {
        self.named_clips.get(&key)
    }
}
