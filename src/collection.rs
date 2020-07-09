use std::collections::HashMap;

// Collection is a general searchable collection of items, such as list of monsters, spells, magical or mundane items.
#[derive(Debug)]
pub struct Collection {
    // List of commands that should be processes by this collection
    // Commands must be unique across all collections!
    pub commands: Vec<&'static str>,
    // List of urls to fetch
    pub urls: Vec<FetchUrl>,
    // Sometime we need one command to be responsible for several collections
    // So, we are using vector here. The first item here is considered to be a "main" collection
    pub collections: Vec<&'static str>,
    // Type of the collection, is needed mostly for formatting
    pub type_: CollectionType,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum CollectionType {
    Item,
    Spell,
    Monster,
}

#[derive(Debug)]

pub struct FetchUrl {
    pub url: &'static str,
    pub field_name: &'static str,
}

lazy_static! {
        // Meta information about our search items
        pub static ref COLLECTIONS: Vec<Collection> = vec![
                Collection {
                    commands: vec!["item", "i"],
                    urls: vec![FetchUrl{url: "items", field_name: "item"}],
                    collections: vec!["item", "baseitem"],
                    type_: CollectionType::Item,
                },
                Collection {
                    commands: vec!["baseitem", "bi"],
                    urls: vec![FetchUrl{url: "items-base", field_name: "baseitem"}],
                    collections: vec!["baseitem"],
                    type_: CollectionType::Item,
                },
                Collection {
                    commands: vec!["spell", "s"],
                    urls: vec![FetchUrl{url: "spells", field_name: "spell"}],
                    collections: vec!["spell"],
                    type_: CollectionType::Spell,
                },
                Collection {
                    commands: vec!["monster", "m"],
                    urls: vec![FetchUrl{url: "bestiary", field_name: "monster"}],
                    collections: vec!["monster"],
                    type_: CollectionType::Monster,
                },
        ];

        // Different view on the above meta that allows quick lookups by different parameters
        pub static ref COMMANDS: HashMap<&'static str, &'static Collection> = {
            let mut map = HashMap::new();
            for item in &*COLLECTIONS {
                for command in &item.commands {
                    map.insert(*command, item);
                }
            }
            map
        };
}

impl Collection {
    pub fn get_default_collection(&self) -> &'static str {
        self.collections[0]
    }
    pub fn get_default_command(&self) -> &'static str {
        self.commands[0]
    }
}
