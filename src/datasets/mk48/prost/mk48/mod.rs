pub mod mk48;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    mk48::register_types(registry);
}
