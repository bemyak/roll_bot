use std::collections::HashMap;

use serde_json::Value as JsonValue;
use simsearch::{SearchOptions, SimSearch};

use crate::collection::CollectionName;

pub struct Cache {
	inner: HashMap<CollectionName, SimSearch<u32>>,
}

impl Cache {
	pub fn new() -> Self {
		Self {
			inner: HashMap::new(),
		}
	}

	pub fn save(&mut self, collection: CollectionName, values: &Vec<JsonValue>) {
		let mut engine = SimSearch::new_with(get_search_options());
		for (i, val) in values.iter().enumerate() {
			let Some(name) = val
				.as_object()
				.map(|v| v.get("name"))
				.flatten()
				.map(JsonValue::as_str)
				.flatten()
			else {
				continue;
			};
			engine.insert(i as u32, name);
		}
		self.inner.insert(collection, engine);
	}

	pub fn search(&self, collection: CollectionName, name: &str) -> Vec<u32> {
		let Some(engine) = self.inner.get(collection) else {
			return vec![];
		};
		engine.search(name)
	}
}

pub fn get_search_options() -> SearchOptions {
	SearchOptions::new()
		.case_sensitive(false)
		.stop_whitespace(false)
		.threshold(0.85)
}
