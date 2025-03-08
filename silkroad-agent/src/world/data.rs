use once_cell::sync::OnceCell;
use pk2::Pk2;
use silkroad_data::characterdata::{load_character_map, RefCharacterData};
use silkroad_data::datamap::DataMap;
use silkroad_data::gold::{load_gold_map, GoldMap};
use silkroad_data::itemdata::{load_item_map, RefItemData};
use silkroad_data::level::{load_level_map, LevelMap};
use silkroad_data::masterydata::{load_mastery_map, RefMasteryData};
use silkroad_data::skilldata::{load_skill_map, RefSkillData};
use silkroad_data::teleport::{
    load_teleport_buildings, load_teleport_links, load_teleport_map, TeleportBuilding, TeleportLink, TeleportLocation,
};
use silkroad_data::FileError;
use std::collections::HashMap;

static ITEMS: OnceCell<DataMap<RefItemData>> = OnceCell::new();
static CHARACTERS: OnceCell<DataMap<RefCharacterData>> = OnceCell::new();
static SKILLS: OnceCell<DataMap<RefSkillData>> = OnceCell::new();
static LEVELS: OnceCell<LevelMap> = OnceCell::new();
static GOLD: OnceCell<GoldMap> = OnceCell::new();
static MASTERIES: OnceCell<DataMap<RefMasteryData>> = OnceCell::new();
static TELEPORTS: OnceCell<HashMap<u16, TeleportLocation>> = OnceCell::new();
static TELEPORT_LINKS: OnceCell<Vec<TeleportLink>> = OnceCell::new();
static TELEPORT_BUILDINGS: OnceCell<DataMap<TeleportBuilding>> = OnceCell::new();

pub struct WorldData;

impl WorldData {
    pub(crate) fn load_data_from(media_pk2: &Pk2) -> Result<(), FileError> {
        let levels = load_level_map(media_pk2)?;
        let gold = load_gold_map(media_pk2)?;
        let characters = load_character_map(media_pk2)?;
        let items = load_item_map(media_pk2)?;
        let skills = load_skill_map(media_pk2)?;
        let masteries = load_mastery_map(media_pk2)?;
        let teleports = load_teleport_map(media_pk2)?;
        let teleport_links = load_teleport_links(media_pk2)?;
        let teleport_buildings = load_teleport_buildings(media_pk2)?;

        let _ = LEVELS.set(levels);
        let _ = GOLD.set(gold);
        let _ = CHARACTERS.set(characters);
        let _ = ITEMS.set(items);
        let _ = SKILLS.set(skills);
        let _ = MASTERIES.set(masteries);
        let _ = TELEPORTS.set(teleports);
        let _ = TELEPORT_LINKS.set(teleport_links);
        let _ = TELEPORT_BUILDINGS.set(teleport_buildings);
        Ok(())
    }

    pub fn items() -> &'static DataMap<RefItemData> {
        ITEMS.get().expect("Items should have been set")
    }

    pub fn characters() -> &'static DataMap<RefCharacterData> {
        CHARACTERS.get().expect("Characters should have been set")
    }

    pub fn skills() -> &'static DataMap<RefSkillData> {
        SKILLS.get().expect("Skills should have been set")
    }

    pub fn levels() -> &'static LevelMap {
        LEVELS.get().expect("Levels should have been set")
    }

    pub fn gold() -> &'static GoldMap {
        GOLD.get().expect("Gold should have been set")
    }

    pub fn masteries() -> &'static DataMap<RefMasteryData> {
        MASTERIES.get().expect("Masteries should have been set")
    }
}
