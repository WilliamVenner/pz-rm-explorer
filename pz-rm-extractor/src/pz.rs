use std::{str::FromStr, collections::{BTreeSet, BTreeMap}};

#[derive(Default, Debug, Serialize)]
pub struct RecordedMediaEffects {
	#[serde(skip_serializing_if = "BTreeMap::is_empty")]
	pub skills: BTreeMap<Skill, i32>,

	#[serde(skip_serializing_if = "BTreeSet::is_empty")]
	pub recipes: BTreeSet<String>
}
impl RecordedMediaEffects {
	pub fn is_empty(&self) -> bool {
		self.skills.is_empty() && self.recipes.is_empty()
	}
}

macro_rules! rm_code_enum {
	{$($abbr:ident => $name:ident => $str:literal),+} => {
		#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
		pub enum Skill {
			$(
				#[serde(rename = $str)]
				$name
			),+
		}

		#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
		pub enum RMCode {
			Skill(Skill, i32),
			Recipe(String)
		}
		impl FromStr for RMCode {
			type Err = ();

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				if s.starts_with("RCP=") {
					Ok(Self::Recipe(s[4..].to_string()))
				} else {
					// Parse skill and amount given/taken
					// e.g. STR+999 STR-999 -> ("STR", 999) ("STR", -999)
					let skill_with_amt = s.split_once("-").and_then(|(name, amt)| {
						amt.parse::<u32>().ok().map(|amt| (name, -(amt as i32)))
					}).or_else(|| {
						s.split_once("+").and_then(|(name, amt)| {
							amt.parse::<u32>().ok().map(|amt| (name, amt as i32))
						})
					});

					let (skill, amt) = match skill_with_amt {
						Some(s) => s,
						None => return Err(())
					};

					match skill {
						$(stringify!($abbr) => Ok(Self::Skill(Skill::$name, amt)),)+
						_ => return Err(())
					}
				}
			}
		}
	};
}

rm_code_enum! {
	SPR => Sprinting => "Sprinting",
	LFT => Lightfooted => "Lightfooted",
	NIM => Nimble => "Nimble",
	SNE => Sneaking => "Sneaking",
	BAA => Axe => "Axe",
	BUA => Blunt => "Blunt",
	CRP => Carpentry => "Carpentry",
	COO => Cooking => "Cooking",
	FRM => Farming => "Farming",
	DOC => FirstAid => "First Aid",
	ELC => Electrical => "Electrical",
	MTL => Metalworking => "Metalworking",
	AIM => Aiming => "Aiming",
	REL => Reloading => "Reloading",
	FIS => Fishing => "Fishing",
	TRA => Trapping => "Trapping",
	FOR => Foraging => "Foraging",
	TAI => Tailoring => "Tailoring",
	MEC => Mechanics => "Mechanics"
}