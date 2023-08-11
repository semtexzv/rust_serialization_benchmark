pub mod mesh;
pub fn register_types(registry: &mut ::protokit::textformat::reflect::Registry) {
    mesh::register_types(registry);
}
