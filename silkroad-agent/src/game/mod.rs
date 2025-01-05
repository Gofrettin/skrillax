use crate::chat::ChatPlugin;
use crate::comp::exp::{Experienced, Leveled, SP};
use crate::comp::gold::GoldPouch;
use crate::comp::inventory::PlayerInventory;
use crate::comp::mastery::MasteryKnowledge;
use crate::comp::player::StatPoints;
use crate::comp::pos::Position;
use crate::comp::skill::{Hotbar, SkillBook};
use crate::comp::{Health, Mana};
use crate::event::{
    DamageReceiveEvent, EntityDeath, LoadingFinishedEvent, PlayerLevelUp, SpawnMonster, UniqueKilledEvent,
};
use crate::ext::ActionIdCounter;
use crate::game::action::handle_action;
use crate::game::damage::{attack_player, handle_damage};
use crate::game::daylight::{advance_daylight, DaylightCycle};
use crate::game::drop::{create_drops, tick_drop, SpawnDrop};
use crate::game::exp::{
    distribute_experience, receive_experience, reset_health_mana_on_level, update_max_hp_mp_on_stat_change,
    ReceiveExperienceEvent,
};
use crate::game::gold::drop_gold;
use crate::game::hotbar::update_hotbar;
use crate::game::inventory::handle_inventory_input;
use crate::game::join::load_finished;
use crate::game::logout::{handle_logout, tick_logout};
use crate::game::mastery::{handle_mastery_levelup, learn_skill};
use crate::game::mind::MindPlugin;
use crate::game::movement::movement_monster;
use crate::game::player_activity::{update_player_activity, PlayerActivity};
use crate::game::spawn::do_spawn_mobs;
use crate::game::stats::increase_stats;
use crate::game::target::{deselect_despawned, player_update_target};
use crate::game::unique::{setup_unique_timers, unique_killed, unique_spawned, update_timers};
use crate::game::visibility::{clear_visibility, player_visibility_update, visibility_update};
use crate::persistence::AppPersistanceExt;
use crate::sync::SynchronizationStage;
use bevy_app::{App, Last, Plugin, PostUpdate, PreUpdate, Startup, Update};
use bevy_ecs::prelude::*;

mod action;
pub(crate) mod attack;
mod damage;
mod daylight;
pub(crate) mod drop;
pub(crate) mod exp;
mod gold;
mod hotbar;
pub(crate) mod inventory;
mod join;
pub(crate) mod logout;
mod mastery;
pub(crate) mod mind;
mod movement;
pub(crate) mod player_activity;
mod spawn;
mod stats;
pub(crate) mod target;
mod unique;
mod visibility;

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChatPlugin)
            .add_plugins(MindPlugin)
            .insert_resource(PlayerActivity::default())
            .insert_resource(DaylightCycle::official())
            .insert_resource(ActionIdCounter::default())
            .add_event::<PlayerLevelUp>()
            .add_event::<LoadingFinishedEvent>()
            .add_event::<UniqueKilledEvent>()
            .add_event::<SpawnDrop>()
            .add_event::<DamageReceiveEvent>()
            .add_event::<EntityDeath>()
            .add_event::<ReceiveExperienceEvent>()
            .add_event::<SpawnMonster>()
            .add_systems(Startup, setup_unique_timers)
            .add_systems(PreUpdate, (update_player_activity, update_timers))
            .add_systems(
                Update,
                (
                    handle_inventory_input,
                    increase_stats,
                    visibility_update,
                    movement_monster,
                    tick_drop,
                    handle_logout,
                    handle_action,
                    tick_logout,
                    player_update_target,
                    deselect_despawned,
                    handle_damage,
                    attack_player,
                    distribute_experience.after(handle_damage),
                    drop_gold.after(handle_damage),
                    receive_experience.after(distribute_experience),
                    reset_health_mana_on_level.after(receive_experience),
                    update_max_hp_mp_on_stat_change.after(increase_stats),
                    handle_mastery_levelup,
                    learn_skill,
                    do_spawn_mobs,
                ),
            )
            .add_systems(
                PostUpdate,
                (
                    player_visibility_update.before(SynchronizationStage::Distribution),
                    load_finished,
                    unique_spawned,
                    unique_killed,
                    advance_daylight,
                    create_drops,
                    update_hotbar,
                ),
            )
            .track_change_component::<Position>()
            .track_change_component::<StatPoints>()
            .track_change_component::<Health>()
            .track_change_component::<Mana>()
            .track_change_component::<Leveled>()
            .track_change_component::<Experienced>()
            .track_change_component::<SP>()
            .track_change_component::<GoldPouch>()
            .track_change_component::<MasteryKnowledge>()
            .track_component::<PlayerInventory>()
            .track_component::<SkillBook>()
            .track_component::<Hotbar>()
            .add_systems(Last, clear_visibility);
    }
}
