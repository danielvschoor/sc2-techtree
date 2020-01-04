//! Terminology:
//! Attack = Single projectile or strike
//! Hit = Attack that connects with the target

use noisy_float::prelude::*;
use serde::{Deserialize, Serialize};

use super::attribute::Attribute;

/// Weapon target type
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum WeaponTargetType {
    /// To ground only
    Ground,
    /// To air only
    Air,
    /// To ground and air both
    Any,
}

/// Weapon bonus
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct WeaponBonus {
    /// Bonus affects attacks against units having this attribute
    pub against: Attribute,
    /// The amount of bonus damage per hit
    pub damage: R32,
}

/// Weapon
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Weapon {
    /// Type of targer(Air, Ground, Any)
    pub target_type: WeaponTargetType,
    /// Damage per hit, no upgrades
    pub damage_per_hit: R32,
    /// Percentage
    pub damage_splash: R32,
    /// Attacks per one attack tick, e.g. 2 for Colossus
    pub attacks: u32,
    /// Range
    pub range: R32,
    /// Cooldown
    pub cooldown: R32,
    /// Bonuses
    pub bonuses: Vec<WeaponBonus>,
}
