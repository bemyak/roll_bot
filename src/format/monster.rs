use super::{get_string_array, Entry, FilterJoinable};
use ejdb::bson::{Bson, Document};

pub trait Monster: Entry {
    fn format_monster(&self) -> Option<String> {
        None
    }
}

impl Monster for Document {}

trait MonsterPrivate: Monster {
    fn get_short_name(&self) -> Option<String>;
    fn get_level(&self) -> Option<i64>;
    fn get_size(&self) -> Option<String>;
    fn get_size_note(&self) -> Option<String>;
    fn get_type(&self) -> Option<String>;
    fn get_alignment(&self) -> Option<String>;
    fn get_ac(&self) -> Option<String>;
    fn get_hp(&self) -> Option<String>;
    fn get_speed(&self) -> Option<String>;
    fn get_str(&self) -> Option<String>;
    fn get_dex(&self) -> Option<String>;
    fn get_con(&self) -> Option<String>;
    fn get_int(&self) -> Option<String>;
    fn get_wis(&self) -> Option<String>;
    fn get_cha(&self) -> Option<String>;
    fn get_save(&self) -> Option<String>;
    fn get_skill(&self) -> Option<String>;
    fn get_senses(&self) -> Option<String>;
    fn get_passive(&self) -> Option<String>;
    fn get_languages(&self) -> Option<String>;
    fn get_cr(&self) -> Option<String>;
    fn get_vulnerable(&self) -> Option<String>;
    fn get_resist(&self) -> Option<String>;
    fn get_immune(&self) -> Option<String>;
    fn get_condition_immune(&self) -> Option<String>;
    fn get_spellcatsing(&self) -> Option<String>;
    fn get_trait(&self) -> Option<String>;
    fn get_action_note(&self) -> Option<String>;
    fn get_action(&self) -> Option<String>;
    fn get_reaction(&self) -> Option<String>;
    fn get_legendary_group(&self) -> Option<String>;
    fn get_legendary_actions(&self) -> Option<String>;
    fn get_legendary_header(&self) -> Option<String>;
    fn get_mythic_header(&self) -> Option<String>;
    fn get_mythic(&self) -> Option<String>;
    fn get_variant(&self) -> Option<String>;
    fn get_is_familiar(&self) -> Option<String>;
    fn get_is_named_creature(&self) -> Option<String>;
    fn get_is_npc(&self) -> Option<String>;
    fn get_environment(&self) -> Option<String>;
    fn get_dragon_casting_color(&self) -> Option<String>;
    fn get_trait_tags(&self) -> Option<String>;
    fn get_action_tags(&self) -> Option<String>;
    fn get_language_tags(&self) -> Option<String>;
    fn get_sense_tags(&self) -> Option<String>;
    fn get_spellcatsing_tags(&self) -> Option<String>;
    fn get_damage_tags(&self) -> Option<String>;
    fn get_misc_tags(&self) -> Option<String>;
    fn get_condition_inflict(&self) -> Option<String>;
    fn get_condition_inflict_legendary(&self) -> Option<String>;
    fn get_condition_inflict_spell(&self) -> Option<String>;
}

impl MonsterPrivate for Document {
    fn get_short_name(&self) -> Option<String> {
        self.get_str("shortName")
            .ok()
            .map(str::to_string)
            .or(self.get_name())
    }
    fn get_level(&self) -> Option<i64> {
        self.get_i64("level").ok()
    }
    fn get_size(&self) -> Option<String> {
        self.get_str("size")
            .ok()
            .map(size_to_string)
            .map(str::to_string)
    }
    fn get_size_note(&self) -> Option<String> {
        self.get_str("sizeNote").map(str::to_string).ok()
    }
    fn get_type(&self) -> Option<String> {
        let type_doc = self.get_document("type").ok()?;
        let type_ = type_doc.get_str("type").ok()?;
        let swarm_size = type_doc.get_str("swarmSize").ok();
        let mut type_ = match swarm_size {
            Some(swarm_size) => format!("swarm of {} {}", size_to_string(swarm_size), type_),
            None => type_.to_string(),
        };

        let tags = get_string_array(self, "tags");
        match tags {
            Some(tags) => {
                type_.push_str(&format!(" ({})", tags.join(", ")));
            }
            None => {}
        };

        Some(type_)
    }
    fn get_alignment(&self) -> Option<String> {
        let arr = self.get_array("alignment").ok()?;
        let result = arr
            .into_iter()
            .filter_map(|element| match element {
                Bson::String(str) => Some(alignment_to_string(str).to_string()),
                Bson::Document(doc) => match doc.get("special") {
                    Some(special) => Some(special.to_string()),
                    None => doc.format_alignment_doc(),
                },
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
    fn get_ac(&self) -> Option<String> {
        todo!()
    }
    fn get_hp(&self) -> Option<String> {
        todo!()
    }
    fn get_speed(&self) -> Option<String> {
        todo!()
    }
    fn get_str(&self) -> Option<String> {
        todo!()
    }
    fn get_dex(&self) -> Option<String> {
        todo!()
    }
    fn get_con(&self) -> Option<String> {
        todo!()
    }
    fn get_int(&self) -> Option<String> {
        todo!()
    }
    fn get_wis(&self) -> Option<String> {
        todo!()
    }
    fn get_cha(&self) -> Option<String> {
        todo!()
    }
    fn get_save(&self) -> Option<String> {
        todo!()
    }
    fn get_skill(&self) -> Option<String> {
        todo!()
    }
    fn get_senses(&self) -> Option<String> {
        todo!()
    }
    fn get_passive(&self) -> Option<String> {
        todo!()
    }
    fn get_languages(&self) -> Option<String> {
        todo!()
    }
    fn get_cr(&self) -> Option<String> {
        todo!()
    }
    fn get_vulnerable(&self) -> Option<String> {
        todo!()
    }
    fn get_resist(&self) -> Option<String> {
        todo!()
    }
    fn get_immune(&self) -> Option<String> {
        todo!()
    }
    fn get_condition_immune(&self) -> Option<String> {
        todo!()
    }
    fn get_spellcatsing(&self) -> Option<String> {
        todo!()
    }
    fn get_trait(&self) -> Option<String> {
        todo!()
    }
    fn get_action_note(&self) -> Option<String> {
        todo!()
    }
    fn get_action(&self) -> Option<String> {
        todo!()
    }
    fn get_reaction(&self) -> Option<String> {
        todo!()
    }
    fn get_legendary_group(&self) -> Option<String> {
        todo!()
    }
    fn get_legendary_actions(&self) -> Option<String> {
        todo!()
    }
    fn get_legendary_header(&self) -> Option<String> {
        todo!()
    }
    fn get_mythic_header(&self) -> Option<String> {
        todo!()
    }
    fn get_mythic(&self) -> Option<String> {
        todo!()
    }
    fn get_variant(&self) -> Option<String> {
        todo!()
    }
    fn get_is_familiar(&self) -> Option<String> {
        todo!()
    }
    fn get_is_named_creature(&self) -> Option<String> {
        todo!()
    }
    fn get_is_npc(&self) -> Option<String> {
        todo!()
    }
    fn get_environment(&self) -> Option<String> {
        todo!()
    }
    fn get_dragon_casting_color(&self) -> Option<String> {
        todo!()
    }
    fn get_trait_tags(&self) -> Option<String> {
        todo!()
    }
    fn get_action_tags(&self) -> Option<String> {
        todo!()
    }
    fn get_language_tags(&self) -> Option<String> {
        todo!()
    }
    fn get_sense_tags(&self) -> Option<String> {
        todo!()
    }
    fn get_spellcatsing_tags(&self) -> Option<String> {
        todo!()
    }
    fn get_damage_tags(&self) -> Option<String> {
        todo!()
    }
    fn get_misc_tags(&self) -> Option<String> {
        todo!()
    }
    fn get_condition_inflict(&self) -> Option<String> {
        todo!()
    }
    fn get_condition_inflict_legendary(&self) -> Option<String> {
        todo!()
    }
    fn get_condition_inflict_spell(&self) -> Option<String> {
        todo!()
    }
}
trait MonsterUtils: Monster {
    fn format_alignment_doc(&self) -> Option<String>;
}

impl MonsterUtils for Document {
    fn format_alignment_doc(&self) -> Option<String> {
        let alignment = self.get_alignment();
        let chance = self
            .get_i64("chance")
            .ok()
            .map(|i| format!("({}% chance)", i));
        let note = self.get_str("note").ok().map(str::to_string);
        vec![alignment, chance, note].filter_join(" ")
    }
}

fn size_to_string(size: &str) -> &str {
    match size {
        "F" => "fine",
        "D" => "diminutive",
        "T" => "tiny",
        "S" => "small",
        "M" => "medium",
        "L" => "large",
        "H" => "huge",
        "G" => "gargantua",
        "C" => "colossal",
        "V" => "varies",
        _ => size,
    }
}

fn alignment_to_string(alignment: &str) -> &str {
    match alignment {
        "L" => "lawful",
        "N" => "neutral",
        "NX" => "neutral (law/chaos axis)",
        "NY" => "neutral (good/evil axis)",
        "C" => "chaotic",
        "G" => "good",
        "E" => "evil",
        "U" => "unaligned",
        "A" => "any alignment",
        _ => alignment,
    }
}
