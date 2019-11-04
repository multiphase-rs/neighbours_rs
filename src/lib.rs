pub mod nbs2d;
pub mod nbs3d;
pub mod particle_array;


pub trait NNPS {
    fn register_particles_to_nnps(&mut self, x: &[f32], y: &[f32], z: &[f32]);
    fn get_neighbours(&self, x: f32, y: f32, z: f32) -> Vec<usize>;
}
