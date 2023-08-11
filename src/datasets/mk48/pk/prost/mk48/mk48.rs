#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityType(pub u32);
#[protoenum]
impl EntityType {
    #[var(0u32, "ARLEIGH_BURKE")]
    pub const ARLEIGH_BURKE: EntityType = EntityType(0u32);
    #[var(1u32, "BISMARCK")]
    pub const BISMARCK: EntityType = EntityType(1u32);
    #[var(2u32, "CLEMENCEAU")]
    pub const CLEMENCEAU: EntityType = EntityType(2u32);
    #[var(3u32, "FLETCHER")]
    pub const FLETCHER: EntityType = EntityType(3u32);
    #[var(4u32, "G5")]
    pub const G5: EntityType = EntityType(4u32);
    #[var(5u32, "IOWA")]
    pub const IOWA: EntityType = EntityType(5u32);
    #[var(6u32, "KOLKATA")]
    pub const KOLKATA: EntityType = EntityType(6u32);
    #[var(7u32, "OSA")]
    pub const OSA: EntityType = EntityType(7u32);
    #[var(8u32, "YASEN")]
    pub const YASEN: EntityType = EntityType(8u32);
    #[var(9u32, "ZUBR")]
    pub const ZUBR: EntityType = EntityType(9u32);
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Vector2f {
    #[field(1u32, "x", fixed32, singular)]
    pub x: f32,
    #[field(2u32, "y", fixed32, singular)]
    pub y: f32,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Transform {
    #[field(1u32, "altitude", varint, singular)]
    pub altitude: i32,
    #[field(2u32, "angle", varint, singular)]
    pub angle: u32,
    #[field(3u32, "position", nested, optional)]
    pub position: Option<Box<Vector2f>>,
    #[field(4u32, "velocity", varint, singular)]
    pub velocity: i32,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Guidance {
    #[field(1u32, "angle", varint, singular)]
    pub angle: u32,
    #[field(2u32, "submerge", bool, singular)]
    pub submerge: bool,
    #[field(3u32, "velocity", varint, singular)]
    pub velocity: i32,
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum ContactOneOfEntityType {
    #[field(3u32, "entity_type", protoenum, raw)]
    EntityType(EntityType),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for ContactOneOfEntityType {
    fn default() -> Self {
        Self::EntityType(Default::default())
    }
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum ContactOneOfPlayerId {
    #[field(5u32, "player_id", varint, raw)]
    PlayerId(u32),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for ContactOneOfPlayerId {
    fn default() -> Self {
        Self::PlayerId(Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Contact {
    #[field(1u32, "damage", varint, singular)]
    pub damage: u32,
    #[field(2u32, "entity_id", varint, singular)]
    pub entity_id: u32,
    #[field(4u32, "guidance", nested, optional)]
    pub guidance: Option<Box<Guidance>>,
    #[field(6u32, "reloads", bool, packed)]
    pub reloads: Vec<bool>,
    #[field(7u32, "transform", nested, optional)]
    pub transform: Option<Box<Transform>>,
    #[field(8u32, "turret_angles", varint, packed)]
    pub turret_angles: Vec<u32>,
    #[oneof([3u32], ["entity_type"])]
    pub _entity_type: Option<ContactOneOfEntityType>,
    #[oneof([5u32], ["player_id"])]
    pub _player_id: Option<ContactOneOfPlayerId>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct ChunkId {
    #[field(1u32, "x", varint, singular)]
    pub x: i32,
    #[field(2u32, "y", varint, singular)]
    pub y: i32,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct TerrainUpdate {
    #[field(1u32, "chunk_id", nested, optional)]
    pub chunk_id: Option<Box<ChunkId>>,
    #[field(2u32, "data", bytes, singular)]
    pub data: Vec<u8>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Update {
    #[field(1u32, "contacts", nested, repeated)]
    pub contacts: Vec<Contact>,
    #[field(2u32, "score", varint, singular)]
    pub score: u32,
    #[field(3u32, "world_radius", fixed32, singular)]
    pub world_radius: f32,
    #[field(4u32, "terrain_updates", nested, repeated)]
    pub terrain_updates: Vec<TerrainUpdate>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Updates {
    #[field(1u32, "updates", nested, repeated)]
    pub updates: Vec<Update>,
}
