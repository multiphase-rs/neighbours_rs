pub mod nbs2d;
pub mod nbs3d;
pub mod bst;
pub mod particle_array;
pub mod prelude;


pub trait NNPS {
    fn register_particles_to_nnps(&mut self, x: &[f64], y: &[f64], z: &[f64]);
    fn get_neighbours(&self, x: f64, y: f64, z: f64) -> Vec<usize>;
}
