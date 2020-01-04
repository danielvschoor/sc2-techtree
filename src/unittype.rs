use noisy_float::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::attribute::Attribute;
use crate::ids::*;
use crate::requirement::Requirement;
use crate::weapon::*;
use crate::Race;

/// Unit or structure
#[derive(Debug, Serialize, Deserialize, Clone, Eq)]
pub struct UnitType {
    /// Id
    pub id: UnitTypeId,
    /// Name
    pub name: String,
    /// Race
    pub race: Race,
    /// Supply
    pub supply: R32,
    /// None for untransportable
    pub cargo_size: Option<u32>,
    /// None if cannot transport units
    pub cargo_capacity: Option<u32>,
    /// Max hp
    pub max_health: u32,
    /// Max shield
    pub max_shield: Option<u32>,
    /// Armor
    pub armor: u32,
    /// Vision range
    pub sight: R32,
    /// None if not detector
    pub detection_range: Option<R32>,
    /// Speed without upgrades of buffs, None if cannot move at all
    pub speed: Option<R32>,
    /// Creep speed multiplier without upgrades of buffs
    pub speed_creep_mul: Option<R32>,
    /// Max energy
    pub max_energy: Option<u32>,
    /// Start energy
    pub start_energy: Option<u32>,
    /// List of weapons, sorted by priority
    pub weapons: Vec<Weapon>,
    /// Attributes
    pub attributes: HashSet<Attribute>,
    /// A list of abilities and their requirements
    pub abilities: Vec<UnitAbilityReq>,
    /// Building size on grid, not available for non-structures
    pub placement_size: Option<u32>,
    /// Radius approximating the size of the unit.
    #[serde(default)] // TODO: remove this and require
    pub radius: R32,
    /// Produces pylon power with this radius
    pub power_radius: Option<R32>,
    /// Terran add-on can be used with this structure
    pub accepts_addon: bool,
    /// Requires a pylon power to function
    pub needs_power: bool,
    /// Requires creep for placement
    pub needs_creep: bool,
    /// Requires a vespene gayser for placement
    pub needs_gayser: bool,
    /// Structure attribute is set
    pub is_structure: bool,
    /// Can be used as an add-on
    pub is_addon: bool,
    /// Workers: Probe, Drone, SCV
    pub is_worker: bool,
    /// Flying buildings not included
    pub is_townhall: bool,
}
impl PartialEq for UnitType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Unit ability with a possible requirement
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UnitAbilityReq {
    ability: AbilityId,
    requirement: Option<Requirement>,
}
