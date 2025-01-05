use chrono::{DateTime, Utc};
use itertools::Itertools;
use silkroad_game_base::Race;
use sqlx::{Error, PgPool};
use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(sqlx::Type, Copy, Clone, Debug)]
#[sqlx(type_name = "race")]
#[sqlx(rename_all = "lowercase")]
pub enum DbRace {
    Chinese,
    European,
}

impl From<Race> for DbRace {
    fn from(value: Race) -> Self {
        match value {
            Race::European => Self::European,
            Race::Chinese => Self::Chinese,
        }
    }
}

impl From<DbRace> for Race {
    fn from(value: DbRace) -> Self {
        match value {
            DbRace::Chinese => Race::Chinese,
            DbRace::European => Race::European,
        }
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct CharacterData {
    pub id: i32,
    pub user_id: i32,
    pub server_id: i32,
    pub charname: String,
    pub race: DbRace,
    pub character_type: i32,
    pub scale: i16,
    pub level: i16,
    pub max_level: i16,
    pub exp: i64,
    pub sp: i32,
    pub sp_exp: i32,
    pub strength: i16,
    pub intelligence: i16,
    pub stat_points: i16,
    pub current_hp: i32,
    pub current_mp: i32,
    pub deletion_end: Option<DateTime<Utc>>,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: i16,
    pub region: i16,
    pub berserk_points: i16,
    pub gold: i64,
    pub beginner_mark: bool,
    pub gm: bool,
    pub last_logout: Option<DateTime<Utc>>,
}

impl CharacterData {
    pub async fn fetch_characters<T: Borrow<PgPool>>(
        user: i32,
        shard: u16,
        pool: T,
    ) -> Result<Vec<CharacterData>, Error> {
        sqlx::query_as!(
            CharacterData,
            "SELECT id, user_id, server_id, character_type, scale, level, exp, strength, intelligence, stat_points, current_hp, current_mp, charname, deletion_end, sp, x, y, z, max_level, region, berserk_points, gold, sp_exp, beginner_mark, gm, last_logout, rotation, race as \"race!: DbRace\" FROM characters WHERE user_id = $1 AND server_id = $2 AND (deletion_end > NOW() OR deletion_end is null) ORDER BY id ASC",
            user,
            shard as i32
        ).fetch_all(pool.borrow()).await
    }

    pub async fn check_name_available<T: Borrow<PgPool>>(name: String, server_id: u16, pool: T) -> (String, bool) {
        let result = sqlx::query!(
            "SELECT COUNT(*) as \"count!\" FROM characters WHERE LOWER(charname) = LOWER($1) and server_id = $2",
            name.clone(),
            server_id as i16
        )
        .fetch_one(pool.borrow())
        .await
        .unwrap();

        (name, result.count == 0)
    }

    pub async fn update_last_played_of<T: Borrow<PgPool>>(character_id: u32, pool: T) {
        sqlx::query!(
            "UPDATE characters SET last_logout = CURRENT_TIMESTAMP WHERE id = $1",
            character_id as i32
        )
        .execute(pool.borrow())
        .await
        .expect("Should be able to update last played.");
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct CharacterItem {
    pub id: i32,
    pub character_id: i32,
    pub item_obj_id: i32,
    pub upgrade_level: i16,
    pub variance: Option<i64>,
    pub slot: i16,
    pub amount: i16,
}

impl CharacterItem {
    pub async fn fetch_bulk_character_items<T: Borrow<PgPool>>(
        character_ids: &[i32],
        pool: T,
    ) -> Result<HashMap<i32, Vec<CharacterItem>>, Error> {
        let all_items: Vec<CharacterItem> = sqlx::query_as!(
            CharacterItem,
            "SELECT * FROM character_items WHERE character_id in (SELECT * FROM UNNEST($1::INTEGER[]))",
            character_ids
        )
        .fetch_all(pool.borrow())
        .await?;

        let character_item_map = all_items.into_iter().into_group_map_by(|item| item.character_id);
        Ok(character_item_map)
    }
}

#[derive(sqlx::FromRow, Copy, Clone)]
pub struct CharacterMastery {
    pub character_id: i32,
    pub mastery_id: i32,
    pub level: i16,
}

impl CharacterMastery {
    pub async fn fetch_for_characters<T: Borrow<PgPool>>(
        character_ids: &[i32],
        pool: T,
    ) -> Result<Vec<CharacterMastery>, Error> {
        let masteries = sqlx::query_as!(
            CharacterMastery,
            "SELECT mastery_id, character_id, level FROM character_masteries WHERE character_id in (SELECT * FROM UNNEST($1::INTEGER[]))",
            character_ids
        )
                .fetch_all(pool.borrow())
                .await?;
        Ok(masteries)
    }
}

#[derive(sqlx::FromRow, Copy, Clone)]
pub struct CharacterSkill {
    pub character_id: i32,
    pub skill_group_id: i32,
    pub level: i16,
}

impl CharacterSkill {
    pub async fn fetch_for_character<T: Borrow<PgPool>>(
        character_ids: &[i32],
        pool: T,
    ) -> Result<Vec<CharacterSkill>, Error> {
        let skills = sqlx::query_as!(
            CharacterSkill,
            "SELECT character_id, skill_group_id, level FROM character_skills WHERE character_id in (SELECT * FROM UNNEST($1::INTEGER[]))",
            character_ids
        )
                .fetch_all(pool.borrow())
                .await?;
        Ok(skills)
    }
}

#[derive(sqlx::FromRow, Copy, Clone)]
pub struct HotbarEntry {
    pub character_id: i32,
    pub slot: i16,
    pub kind: i16,
    pub data: i32,
}

pub struct CharacterHotbar;
impl CharacterHotbar {
    pub async fn fetch_hotbar_entries<T: Borrow<PgPool>>(
        character_ids: &[i32],
        pool: T,
    ) -> Result<Vec<HotbarEntry>, Error> {
        let entries = sqlx::query_as!(
            HotbarEntry,
            "SELECT character_id, slot, kind, data FROM hotbar_entries WHERE character_id in (SELECT * FROM UNNEST($1::INTEGER[]))",
            character_ids
        )
        .fetch_all(pool.borrow())
        .await?;
        Ok(entries)
    }
}
