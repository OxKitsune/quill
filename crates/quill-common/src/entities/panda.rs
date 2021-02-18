use bytemuck::{Zeroable, Pod};
/// Marker component for panda entities.
///
/// # Example
/// A system that queries for all pandas:
/// ```no_run
/// use quill::{Game, Position, entities::Panda};
/// # struct MyPlugin;
/// fn print_entities_system(_plugin: &mut MyPlugin, game: &mut Game) {
///     for (entity, (position, _)) in game.query::<(&Position, &Panda)>() {
///         println!("Found a panda with position {:?}", position);
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Panda;

pod_component_impl!(Panda);