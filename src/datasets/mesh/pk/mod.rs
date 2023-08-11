pub mod prost;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    prost::register_types(registry);
}
