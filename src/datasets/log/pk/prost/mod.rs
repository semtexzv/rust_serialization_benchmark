pub mod log;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    log::register_types(registry);
}
