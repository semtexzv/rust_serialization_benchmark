pub mod minecraft_savedata;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    minecraft_savedata::register_types(registry);
}
