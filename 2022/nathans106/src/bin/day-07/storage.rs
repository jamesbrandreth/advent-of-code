use std::collections::HashMap;

pub trait Storage {
    fn name(&self) -> &String;
    fn size(&self) -> i32;
    fn is_file(&self) -> bool;
}

pub struct Directory {
    name: String,
    items: HashMap<String, Box<dyn Storage>>,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Directory {
            name,
            items: HashMap::new(),
        }
    }

    pub fn insert(&mut self, item: Box<dyn Storage>) {
        self.items.insert(item.name().to_string(), item);
    }
}

impl Storage for Directory {
    fn name(&self) -> &String {
        &self.name
    }

    fn size(&self) -> i32 {
        self.items.values().map(|item| item.size()).sum()
    }

    fn is_file(&self) -> bool {
        false
    }
}

pub struct File {
    name: String,
    size: i32,
}

impl File {
    pub fn new(name: String, size: i32) -> Self {
        Self { name, size }
    }
}

impl Storage for File {
    fn name(&self) -> &String {
        &self.name
    }

    fn size(&self) -> i32 {
        self.size
    }

    fn is_file(&self) -> bool {
        true
    }
}
