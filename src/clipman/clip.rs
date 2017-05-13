use chrono::prelude::*;

pub struct Clip {
    created: DateTime<Local>,
    modified: DateTime<Local>,
    title: String,
    key: Option<String>,
    content: String,
}

impl Clip {
    pub fn new() -> Self {
        Clip {
            created: Local::now(),
            modified: Local::now(),
            title: String::from(""),
            key: None,
            content: String::from(""),
        }
    }

    pub fn set_content(&mut self, content: String) {
        self.modified = Local::now();
        self.content = content;
    }

    pub fn set_title(&mut self, title: String) {
        self.modified = Local::now();
        self.title = title;
    }

    pub fn set_key(&mut self, key: String) {
        self.modified = Local::now();
        self.key = Some(key);
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn key(&self) -> Option<String> {
        self.key.clone()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn created(&self) -> DateTime<Local> {
        self.created.clone()
    }

    pub fn modified(&self) -> DateTime<Local> {
        self.modified.clone()
    }
}


pub struct ClipBuilder {
    pub title: String,
    pub key: Option<String>,
    pub content: String,
}

impl ClipBuilder {
    pub fn new() -> Self {
        ClipBuilder {
            title: String::from(""),
            key: None,
            content: String::from(""),
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    pub fn key(mut self, key: String) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self) -> Clip {
        Clip {
            created: Local::now(),
            modified: Local::now(),
            title: self.title,
            key: self.key,
            content: self.content,
        }
    }
}


