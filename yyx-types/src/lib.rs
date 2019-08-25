//! 痒痒熊生成的快照的类型定义
//!

#![warn(clippy::all)]

#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate yyx_utils;

use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};

pub mod result;

mod hero;
mod hero_book_shard;
mod hero_equip;
mod hero_equip_preset;
mod player;
mod realm_card;
mod story_task;

pub use self::hero::*;
pub use self::hero_book_shard::*;
pub use self::hero_equip::*;
pub use self::hero_equip_preset::*;
pub use self::player::*;
pub use self::realm_card::*;
pub use self::story_task::*;

/// 痒痒熊生成的快照
#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
  pub version: String,
  pub timestamp: DateTime<Local>,
  pub data: SnapshotData,
  pub cbg_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotData {
  /// 玩家信息
  pub player: Player,
  /// 玩家货币
  pub currency: PlayerCurrency,
  /// 所有的式神
  pub heroes: Vec<Hero>,
  /// 所有的御魂
  pub hero_equips: Vec<HeroEquip>,
  /// 所有的御魂方案
  pub hero_equip_presets: Vec<HeroEquipPreset>,
  /// 所有的式神召唤书碎片
  pub hero_book_shards: Vec<HeroBookShard>,
  /// 所有的结界卡
  #[serde(default)]
  pub realm_cards: Vec<RealmCard>,
  #[serde(default)]
  pub story_tasks: Vec<StoryTask>,
}
