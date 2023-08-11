#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameType(pub u32);
#[protoenum]
impl GameType {
    #[var(0u32, "SURVIVAL")]
    pub const SURVIVAL: GameType = GameType(0u32);
    #[var(1u32, "CREATIVE")]
    pub const CREATIVE: GameType = GameType(1u32);
    #[var(2u32, "ADVENTURE")]
    pub const ADVENTURE: GameType = GameType(2u32);
    #[var(3u32, "SPECTATOR")]
    pub const SPECTATOR: GameType = GameType(3u32);
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Item {
    #[field(1u32, "count", varint, singular)]
    pub count: i32,
    #[field(2u32, "slot", varint, singular)]
    pub slot: u32,
    #[field(3u32, "id", string, singular)]
    pub id: String,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Abilities {
    #[field(1u32, "walk_speed", fixed32, singular)]
    pub walk_speed: f32,
    #[field(2u32, "fly_speed", fixed32, singular)]
    pub fly_speed: f32,
    #[field(3u32, "may_fly", bool, singular)]
    pub may_fly: bool,
    #[field(4u32, "flying", bool, singular)]
    pub flying: bool,
    #[field(5u32, "invulnerable", bool, singular)]
    pub invulnerable: bool,
    #[field(6u32, "may_build", bool, singular)]
    pub may_build: bool,
    #[field(7u32, "instabuild", bool, singular)]
    pub instabuild: bool,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Vector3d {
    #[field(1u32, "x", fixed64, singular)]
    pub x: f64,
    #[field(2u32, "y", fixed64, singular)]
    pub y: f64,
    #[field(3u32, "z", fixed64, singular)]
    pub z: f64,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Vector2f {
    #[field(1u32, "x", fixed32, singular)]
    pub x: f32,
    #[field(2u32, "y", fixed32, singular)]
    pub y: f32,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Uuid {
    #[field(1u32, "x0", varint, singular)]
    pub x0: u32,
    #[field(2u32, "x1", varint, singular)]
    pub x1: u32,
    #[field(3u32, "x2", varint, singular)]
    pub x2: u32,
    #[field(4u32, "x3", varint, singular)]
    pub x3: u32,
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum EntityOneOfCustomName {
    #[field(13u32, "custom_name", string, raw)]
    CustomName(String),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for EntityOneOfCustomName {
    fn default() -> Self {
        Self::CustomName(Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Entity {
    #[field(1u32, "id", string, singular)]
    pub id: String,
    #[field(2u32, "pos", nested, optional)]
    pub pos: Option<Box<Vector3d>>,
    #[field(3u32, "motion", nested, optional)]
    pub motion: Option<Box<Vector3d>>,
    #[field(4u32, "rotation", nested, optional)]
    pub rotation: Option<Box<Vector2f>>,
    #[field(5u32, "fall_distance", fixed32, singular)]
    pub fall_distance: f32,
    #[field(6u32, "fire", varint, singular)]
    pub fire: u32,
    #[field(7u32, "air", varint, singular)]
    pub air: u32,
    #[field(8u32, "on_ground", bool, singular)]
    pub on_ground: bool,
    #[field(9u32, "no_gravity", bool, singular)]
    pub no_gravity: bool,
    #[field(10u32, "invulnerable", bool, singular)]
    pub invulnerable: bool,
    #[field(11u32, "portal_cooldown", varint, singular)]
    pub portal_cooldown: i32,
    #[field(12u32, "uuid", nested, optional)]
    pub uuid: Option<Box<Uuid>>,
    #[field(14u32, "custom_name_visible", bool, singular)]
    pub custom_name_visible: bool,
    #[field(15u32, "silent", bool, singular)]
    pub silent: bool,
    #[field(16u32, "glowing", bool, singular)]
    pub glowing: bool,
    #[oneof([13u32], ["custom_name"])]
    pub _custom_name: Option<EntityOneOfCustomName>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct RecipeBook {
    #[field(1u32, "recipes", string, repeated)]
    pub recipes: Vec<String>,
    #[field(2u32, "to_be_displayed", string, repeated)]
    pub to_be_displayed: Vec<String>,
    #[field(3u32, "is_filtering_craftable", bool, singular)]
    pub is_filtering_craftable: bool,
    #[field(4u32, "is_gui_open", bool, singular)]
    pub is_gui_open: bool,
    #[field(5u32, "is_furnace_filtering_craftable", bool, singular)]
    pub is_furnace_filtering_craftable: bool,
    #[field(6u32, "is_furnace_gui_open", bool, singular)]
    pub is_furnace_gui_open: bool,
    #[field(7u32, "is_blasting_furnace_filtering_craftable", bool, singular)]
    pub is_blasting_furnace_filtering_craftable: bool,
    #[field(8u32, "is_blasting_furnace_gui_open", bool, singular)]
    pub is_blasting_furnace_gui_open: bool,
    #[field(9u32, "is_smoker_filtering_craftable", bool, singular)]
    pub is_smoker_filtering_craftable: bool,
    #[field(10u32, "is_smoker_gui_open", bool, singular)]
    pub is_smoker_gui_open: bool,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Vehicle {
    #[field(1u32, "uuid", nested, optional)]
    pub uuid: Option<Box<Uuid>>,
    #[field(2u32, "entity", nested, optional)]
    pub entity: Option<Box<Entity>>,
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum PlayerOneOfSpawnDimension {
    #[field(7u32, "spawn_dimension", string, raw)]
    SpawnDimension(String),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for PlayerOneOfSpawnDimension {
    fn default() -> Self {
        Self::SpawnDimension(Default::default())
    }
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum PlayerOneOfSpawnForced {
    #[field(11u32, "spawn_forced", bool, raw)]
    SpawnForced(bool),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for PlayerOneOfSpawnForced {
    fn default() -> Self {
        Self::SpawnForced(Default::default())
    }
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum PlayerOneOfEnteredNetherPosition {
    #[field(23u32, "entered_nether_position", nested, raw)]
    EnteredNetherPosition(Vector3d),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for PlayerOneOfEnteredNetherPosition {
    fn default() -> Self {
        Self::EnteredNetherPosition(Default::default())
    }
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum PlayerOneOfRootVehicle {
    #[field(24u32, "root_vehicle", nested, raw)]
    RootVehicle(Vehicle),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for PlayerOneOfRootVehicle {
    fn default() -> Self {
        Self::RootVehicle(Default::default())
    }
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum PlayerOneOfShoulderEntityLeft {
    #[field(25u32, "shoulder_entity_left", nested, raw)]
    ShoulderEntityLeft(Entity),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for PlayerOneOfShoulderEntityLeft {
    fn default() -> Self {
        Self::ShoulderEntityLeft(Default::default())
    }
}
#[derive(Debug, Clone, PartialEq, Proto)]
pub enum PlayerOneOfShoulderEntityRight {
    #[field(26u32, "shoulder_entity_right", nested, raw)]
    ShoulderEntityRight(Entity),
    __Unused(::core::marker::PhantomData<&'static ()>),
}
impl Default for PlayerOneOfShoulderEntityRight {
    fn default() -> Self {
        Self::ShoulderEntityRight(Default::default())
    }
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Player {
    #[field(1u32, "game_type", protoenum, singular)]
    pub game_type: GameType,
    #[field(2u32, "previous_game_type", protoenum, singular)]
    pub previous_game_type: GameType,
    #[field(3u32, "score", varint, singular)]
    pub score: i64,
    #[field(4u32, "dimension", string, singular)]
    pub dimension: String,
    #[field(5u32, "selected_item_slot", varint, singular)]
    pub selected_item_slot: u32,
    #[field(6u32, "selected_item", nested, optional)]
    pub selected_item: Option<Box<Item>>,
    #[field(8u32, "spawn_x", varint, singular)]
    pub spawn_x: i64,
    #[field(9u32, "spawn_y", varint, singular)]
    pub spawn_y: i64,
    #[field(10u32, "spawn_z", varint, singular)]
    pub spawn_z: i64,
    #[field(12u32, "sleep_timer", varint, singular)]
    pub sleep_timer: u32,
    #[field(13u32, "food_exhaustion_level", fixed32, singular)]
    pub food_exhaustion_level: f32,
    #[field(14u32, "food_saturation_level", fixed32, singular)]
    pub food_saturation_level: f32,
    #[field(15u32, "food_tick_timer", varint, singular)]
    pub food_tick_timer: u32,
    #[field(16u32, "xp_level", varint, singular)]
    pub xp_level: u32,
    #[field(17u32, "xp_p", fixed32, singular)]
    pub xp_p: f32,
    #[field(18u32, "xp_total", varint, singular)]
    pub xp_total: i32,
    #[field(19u32, "xp_seed", varint, singular)]
    pub xp_seed: i32,
    #[field(20u32, "inventory", nested, repeated)]
    pub inventory: Vec<Item>,
    #[field(21u32, "ender_items", nested, repeated)]
    pub ender_items: Vec<Item>,
    #[field(22u32, "abilities", nested, optional)]
    pub abilities: Option<Box<Abilities>>,
    #[field(27u32, "seen_credits", bool, singular)]
    pub seen_credits: bool,
    #[field(28u32, "recipe_book", nested, optional)]
    pub recipe_book: Option<Box<RecipeBook>>,
    #[oneof([7u32], ["spawn_dimension"])]
    pub _spawn_dimension: Option<PlayerOneOfSpawnDimension>,
    #[oneof([11u32], ["spawn_forced"])]
    pub _spawn_forced: Option<PlayerOneOfSpawnForced>,
    #[oneof([23u32], ["entered_nether_position"])]
    pub _entered_nether_position: Option<PlayerOneOfEnteredNetherPosition>,
    #[oneof([24u32], ["root_vehicle"])]
    pub _root_vehicle: Option<PlayerOneOfRootVehicle>,
    #[oneof([25u32], ["shoulder_entity_left"])]
    pub _shoulder_entity_left: Option<PlayerOneOfShoulderEntityLeft>,
    #[oneof([26u32], ["shoulder_entity_right"])]
    pub _shoulder_entity_right: Option<PlayerOneOfShoulderEntityRight>,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Players {
    #[field(1u32, "players", nested, repeated)]
    pub players: Vec<Player>,
}
