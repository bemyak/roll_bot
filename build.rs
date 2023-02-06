use std::{
	fs::File,
	io::Read,
	path::{Path, PathBuf},
};

fn main() {
	if cfg!(feature = "regenerate_types") {
		regenerate_types()
	}
}

fn regenerate_types() {
	let data_path = Path::new("../data/test/schema/");
	let gen_path = Path::new("src/format/gen/");
	let _ = std::fs::remove_dir_all(gen_path);
	std::fs::create_dir_all(gen_path).unwrap();

	gen_types(
		data_path.join("entry.json").as_path(),
		gen_path.join("entry.rs").as_path(),
		Some("EntryJson"),
	);
	gen_types(
		data_path.join("util.json").as_path(),
		gen_path.join("util.rs").as_path(),
		None,
	);
	gen_types(
		data_path.join("trapshazards.json").as_path(),
		gen_path.join("trapshazards.rs").as_path(),
		None,
	);

	gen_types(
		data_path.join("bestiary/bestiary.json").as_path(),
		gen_path.join("bestiary.rs").as_path(),
		None,
	);
	gen_types(
		data_path.join("spells/spells.json").as_path(),
		gen_path.join("spells.rs").as_path(),
		None,
	);
	gen_types(
		data_path.join("objects.json").as_path(),
		gen_path.join("objects.rs").as_path(),
		None,
	);
	gen_types(
		data_path.join("items.json").as_path(),
		gen_path.join("items.rs").as_path(),
		None,
	);

	gen_mod(gen_path);
	rustfmt(gen_path);
	replace(
		gen_path.join("entry.rs").as_path(),
		"EntryJson = serde_json::Value",
		"EntryJson = Entry",
	);
}

fn replace(file: &Path, from: &str, to: &str) {
	let mut f = File::open(file).unwrap();
	let mut data = String::new();
	f.read_to_string(&mut data).unwrap();
	drop(f);

	let new_data = data.replace(from, to);
	std::fs::write(file, new_data.as_bytes()).unwrap();
}

fn gen_types(input_file: &Path, output_file: &Path, type_name: Option<&str>) {
	let entry = schemafy_lib::Generator::builder()
		.with_root_name(type_name.map(str::to_owned))
		.with_input_file(input_file.to_str().unwrap())
		.build()
		.generate();
	std::fs::write(
		output_file,
		format!("use serde::{{Deserialize, Serialize}};\nuse super::*;\n{entry}"),
	)
	.unwrap();
}

fn gen_mod(gen_path: &Path) {
	let m = std::fs::read_dir(gen_path)
		.unwrap()
		.into_iter()
		.filter_map(Result::ok)
		.map(|t| t.path())
		.map(|path: PathBuf| {
			format!(
				"pub mod {0};\npub use {0}::*;",
				path.with_extension("")
					.file_name()
					.unwrap()
					.to_string_lossy()
			)
		})
		.collect::<Vec<_>>()
		.join("\n");
	std::fs::write(gen_path.join("mod.rs"), m).unwrap();
}

fn rustfmt(gen_path: &Path) {
	let paths = std::fs::read_dir(gen_path).unwrap();
	for path in paths {
		let path = path.unwrap().path();
		std::process::Command::new("rustfmt")
			.arg(path)
			.output()
			.unwrap();
	}
}
