use std::{collections::BTreeMap, time::Instant, str::FromStr};

#[macro_use] extern crate serde;
#[macro_use] mod util;

mod pz;
use pz::*;

fn extract_rm_names(rm_names: String) -> BTreeMap<String, String> {
	let mut extracted = BTreeMap::new();

	let re_rm_names = regex::RegexBuilder::new(r#"^(RM_.+?)\s*=\s*"(.+)"\s*$"#).multi_line(true).build().unwrap();

	for cap in re_rm_names.captures_iter(&rm_names) {
		let name = cap.get(1).unwrap().as_str().to_string();

		let text = cap.get(2).unwrap().as_str();
		let text = text.replace("\\\"", "\""); // Just in case we encounter any escaped quotation marks

		extracted.insert(
			name,
			text
		);
	}

	assert!(!extracted.is_empty(), "Failed to extract any RM names from translation file");

	extracted
}

fn extract_recorded_media<'a>(rm_names: &'a BTreeMap<String, String>, recorded_media: String) -> BTreeMap<&'a String, RecordedMediaEffects> {
	rlua::Lua::new().context(|ctx| {
		let mut extracted = BTreeMap::new();

		ctx.load(&recorded_media)
			.set_name("recorded_media.lua").unwrap()
			.exec().expect("Failed to run recorded_media.lua!");

		let global_table = ctx.globals().get::<_, rlua::Table>("RecMedia").expect("RecMedia table doesn't exist in _G");

		for (id, recorded_media) in global_table.pairs::<String, rlua::Table>().map(Result::unwrap) {
			// get `itemDisplayName` from the table
			let item_display_name = or_continue!(
				recorded_media.get::<_, String>("itemDisplayName"); ??
				eprintln!("RecMedia[{id:?}] doesn't have `itemDisplayName`")
			);

			// look it up in the translation string dictionary
			let item_display_name = or_continue!(
				rm_names.get(&item_display_name); ??
				eprintln!("Failed to find RM name for RecMedia[{id:?}]")
			);

			// get the `lines` list
			let lines = or_continue!(
				recorded_media.get::<_, rlua::Table>("lines"); ??
				eprintln!("RecMedia[{id:?}] doesn't have `lines`")
			);

			let mut effects = RecordedMediaEffects::default();
			for line in lines.sequence_values::<rlua::Table>().map(Result::unwrap) {
				for code in or_continue!(line.get::<_, String>("codes")).split(",") {
					match RMCode::from_str(code) {
						Err(_) => continue,

						Ok(RMCode::Recipe(recipe)) => {
							// found a recipe
							effects.recipes.insert(recipe);
						}

						Ok(RMCode::Skill(skill, amount)) => {
							// found a skill with a +amount or -amount, add it to the map
							*effects.skills.entry(skill).or_insert(0) += amount;
						}
					}
				}
			}

			if !effects.is_empty() {
				extracted.insert(item_display_name, effects);
			}
		}

		extracted
	})
}

fn main() {
	#[cfg(not(debug_assertions))]
	let mut output = std::fs::File::create(std::env::args().nth(1).expect("No output file specified")).expect("Failed to open output file");

	let rm_names = std::fs::read_to_string(pz_path!("media/lua/shared/Translate/EN/Recorded_Media_EN.txt")).expect("Failed to read Recorded_Media_EN.txt");
	let recorded_media = std::fs::read_to_string(pz_path!("media/lua/shared/RecordedMedia/recorded_media.lua")).expect("Failed to read recorded_media.lua");

	let now = Instant::now();

	let rm_names = extract_rm_names(rm_names);
	let extracted = extract_recorded_media(&rm_names, recorded_media);

	println!("done in {:?}", now.elapsed());

	#[cfg(debug_assertions)] {
		println!("{}", serde_json::to_string_pretty(&extracted).expect("Failed to serialize to JSON"));
	}
	#[cfg(not(debug_assertions))] {
		std::io::Write(&mut output, serde_json::to_string(&extracted).expect("Failed to serialize to JSON"));
	}
}
