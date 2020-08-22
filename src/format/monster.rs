use super::{Entry, EntryArrayUtils, EntryUtils, FilterJoinable, Optionable};
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

        let tags = type_doc.get_array_of("tags", Bson::as_str);
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
        result.to_option()
    }
    fn get_ac(&self) -> Option<String> {
        let acs = self.get_array_of("ac", Bson::as_document)?;
        acs.into_iter()
            .filter_map(|doc| doc.format_ac_doc())
            .collect::<Vec<_>>()
            .join(" or ")
            .to_option()
    }
    fn get_hp(&self) -> Option<String> {
        let hp = self.get_document("hp").ok()?;
        let special = hp.get_string("special");
        if special.is_some() {
            return special;
        }
        let avg = hp.get_i64("average").ok().map(|i| i.to_string());
        let formula = hp.get_str("formula").ok().map(|s| format!("({})", s));
        vec![avg, formula].filter_join(" ")
    }
    fn get_speed(&self) -> Option<String> {
        match self.get("speed")? {
            Bson::Document(doc) => vec!["walk", "borrow", "climb", "fly", "swim"]
                .into_iter()
                .filter_map(|key| match doc.get(key) {
                    Some(value) => match value {
                        Bson::Document(doc) => doc.format_speed_val(),
                        Bson::I64(i) => Some(i.to_string()),
                        _ => None,
                    },
                    None => None,
                })
                .collect::<Vec<_>>()
                .join(", ")
                .to_option(),
            Bson::String(s) => Some(s.to_string()),
            Bson::I64(i) => Some(i.to_string()),
            _ => None,
        }
    }
    fn get_str(&self) -> Option<String> {
        self.get_stat("str")
    }
    fn get_dex(&self) -> Option<String> {
        self.get_stat("dex")
    }
    fn get_con(&self) -> Option<String> {
        self.get_stat("con")
    }
    fn get_int(&self) -> Option<String> {
        self.get_stat("int")
    }
    fn get_wis(&self) -> Option<String> {
        self.get_stat("wis")
    }
    fn get_cha(&self) -> Option<String> {
        self.get_stat("cha")
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
    fn format_ac_doc(&self) -> Option<String>;
    fn format_speed_val(&self) -> Option<String>;
    fn get_stat(&self, stat: &str) -> Option<String>;
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
    fn format_ac_doc(&self) -> Option<String> {
        let ac = self.get_i64("ac").ok().map(|i| i.to_string());
        let from = self
            .get_array_of("from", Bson::as_str)
            .map(|arr| format!("({})", arr.join(", ")));
        let condition = self.get_string("condition");
        vec![ac, from, condition].filter_join(" ")
    }
    fn format_speed_val(&self) -> Option<String> {
        let number = self.get_i64("number").ok()?;
        let condition = self.get_str("condition").ok()?;
        Some(format!("{} {}", number, condition))
    }
    fn get_stat(&self, stat: &str) -> Option<String> {
        let num = self.get_i64(stat).ok()?;
        let bonus = (num - 10) / 2;
        if bonus > 0 {
            Some(format!("{} (+{})", num, bonus))
        } else {
            Some(format!("{} ({})", num, bonus))
        }
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
