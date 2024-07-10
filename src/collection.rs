use std::collections::HashMap;

pub type CollectionName = &'static str;
pub type CommandName = &'static str;
pub type Url = &'static str;

// Collection is a general searchable collection of items, such as list of monsters, spells, magical or mundane items.
#[derive(PartialEq, Eq, Debug)]
pub struct Collection {
	// List of commands that should be processes by this collection
	// Commands must be unique across all collections!
	pub commands: &'static [CommandName],
	// List of urls to fetch
	pub urls: &'static [Url],
	// Sometimes we need one command to be responsible for several collections
	// Should be used only in telegram mod!
	pub collections: &'static [CollectionName],
	// Type of the collection, is needed mostly for formatting
	pub type_: CollectionType,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum CollectionType {
	Item,
	Spell,
	Monster,
}

pub const COLLECTIONS: &[Collection] = &[
	Collection {
		commands: &["item", "i"],
		urls: &["items"],
		collections: &["item", "baseitem"],
		type_: CollectionType::Item,
	},
	Collection {
		commands: &["baseitem", "bi"],
		urls: &["items-base"],
		collections: &["baseitem"],
		type_: CollectionType::Item,
	},
	Collection {
		commands: &["spell", "s"],
		urls: &["spells", "spells/sources.json"],
		collections: &["spell"],
		type_: CollectionType::Spell,
	},
	Collection {
		commands: &["monster", "m", "mm"],
		urls: &["bestiary", "bestiary/legendarygroups.json"],
		collections: &["monster"],
		type_: CollectionType::Monster,
	},
];

lazy_static! {
		// Different view on the above meta that allows quick lookups by different parameters
		pub static ref COMMANDS: HashMap<CommandName, &'static Collection> = {
			let mut map = HashMap::new();
			for item in COLLECTIONS {
				for command in item.commands {
					map.insert(*command, item);
				}
			}
			map
		};

		pub static ref COLLECTION_NAMES: Vec<CollectionName> = {
			COLLECTIONS
			.iter().flat_map(|c| c.collections)
			.copied()
			.collect::<Vec<_>>()
		};
}

impl Collection {
	pub fn get_default_command(&self) -> CommandName {
		self.commands[0]
	}
}
