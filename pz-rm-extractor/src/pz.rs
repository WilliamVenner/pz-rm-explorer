use std::{str::FromStr, collections::{BTreeSet, BTreeMap}};

#[derive(Default, Debug, Serialize)]
pub struct RecordedMediaEffects {
	pub skills: BTreeMap<Skill, i32>,
	pub recipes: BTreeSet<String>
}
impl RecordedMediaEffects {
	pub fn is_empty(&self) -> bool {
		self.skills.is_empty() && self.recipes.is_empty()
	}
}

macro_rules! rm_code_enum {
	{$($abbr:ident => $name:ident),+} => {
		#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
		pub enum Skill {
			$($name),+
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
	SPR => Sprinting,
	LFT => Lightfooted,
	NIM => Nimble,
	SNE => Sneaking,
	BAA => Axe,
	BUA => Blunt,
	CRP => Carpentry,
	COO => Cooking,
	FRM => Farming,
	DOC => FirstAid,
	ELC => Electricty,
	MTL => Metalworking,
	AIM => Aiming,
	REL => Reloading,
	FIS => Fishing,
	TRA => Trapping,
	FOR => Foraging,
	TAI => Tailoring,
	MEC => Mechanics
}