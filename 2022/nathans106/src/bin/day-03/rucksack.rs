#[derive(PartialEq, Eq, Debug)]
pub struct Rucksack {
    section1: String,
    section2: String,
}

impl Rucksack {
    pub fn new(section1: String, section2: String) -> Self {
        Rucksack { section1, section2 }
    }
}
