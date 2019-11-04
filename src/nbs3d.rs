use crate::NNPS;

#[derive(Debug, Clone)]
pub struct NBS3D {
    pub head: Vec<usize>,
    pub next: Vec<usize>,
    pub no_x_cells: usize,
    pub no_y_cells: usize,
    pub no_z_cells: usize,
    pub total_no_cells: usize,
    pub cell_size: f32,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
}

impl NBS3D {
    pub fn new(
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
        z_min: f32,
        z_max: f32,
        cell_size: f32,
    ) -> NBS3D {
        // check if the domain size is greater than the cell size
        if cell_size > (x_max - x_min) || cell_size > (y_max - y_min) || cell_size > (z_max - z_min)
        {
            panic!(
                "cell size cannot be greater than the simulation domain, please set your cell size"
            );
        }
        let no_x_cells = ((x_max - x_min) / cell_size) as usize;
        let no_y_cells = ((y_max - y_min) / cell_size) as usize;
        let no_z_cells = ((z_max - z_min) / cell_size) as usize;
        let total_no_cells = no_x_cells * no_y_cells * no_z_cells;
        NBS3D {
            head: vec![usize::max_value(); total_no_cells],
            next: vec![],
            no_x_cells: no_x_cells,
            no_y_cells: no_y_cells,
            no_z_cells: no_z_cells,
            total_no_cells: total_no_cells,
            cell_size: cell_size,
            x_min: x_min,
            x_max: x_max,
            y_min: y_min,
            y_max: y_max,
            z_min: z_min,
            z_max: z_max,
        }
    }
    pub fn initialize_next(&mut self, no_of_particles: usize) {
        self.next = vec![usize::max_value(); no_of_particles];
    }

    pub fn from_limits_and_no_of_particles(
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
        z_min: f32,
        z_max: f32,
        cell_size: f32,
        no_of_particles: usize,
    ) {
        let mut nbs3d = NBS3D::new(x_min, x_max, y_min, y_max, z_min, z_max, cell_size);
        nbs3d.initialize_next(no_of_particles);
    }
}

impl NNPS for NBS3D {
    fn register_particles_to_nnps(&mut self, x: &[f32], y: &[f32], z: &[f32]) {
        let max_value = usize::max_value();

        let x_min = self.x_min;
        let x_max = self.x_max;
        let y_min = self.y_min;
        let y_max = self.y_max;
        let z_min = self.z_min;
        let z_max = self.z_max;
        let cell_size = self.cell_size;
        let no_x_cells = self.no_x_cells;
        let no_y_cells = self.no_y_cells;
        let total_no_cells = self.total_no_cells;

        let head = &mut self.head;
        let next = &mut self.next;

        // clear the previous stacked indices
        for i in 0..head.len() {
            head[i] = max_value;
        }
        // similarly for next
        for i in 0..next.len() {
            next[i] = max_value;
        }

        for i in 0..x.len() {
            if (x[i] >= x_min && x[i] <= x_max)
                && (y[i] >= y_min && y[i] <= y_max)
                && (z[i] >= z_min && z[i] <= z_max)
            {
                // eliminate the particles which are out of domain
                // get the index of the particle
                let nx = ((x[i] - x_min) / cell_size) as usize;
                let ny = ((y[i] - y_min) / cell_size) as usize;
                let nz = ((z[i] - z_min) / cell_size) as usize;

                let idx = nz * no_y_cells + ny * no_x_cells + nx;

                if idx < total_no_cells {
                    next[i] = head[idx];
                    head[idx] = i;
                }
            }
        }
    }

    fn get_neighbours(&self, x: f32, y: f32, z: f32) -> Vec<usize> {
        let mut neighbours: Vec<usize> = vec![];
        let mut particle_idx;
        let head = &self.head;
        let next = &self.next;
        let usize_max_value = usize::max_value();
        let x_min = self.x_min;
        let x_max = self.x_max;
        let y_min = self.y_min;
        let y_max = self.y_max;
        let z_min = self.z_min;
        let z_max = self.z_max;
        let cell_size = self.cell_size;
        let no_x_cells = self.no_x_cells;
        let no_y_cells = self.no_y_cells;
        let total_no_cells = self.total_no_cells;

        // check if the particle is in the simulation domain
        if (x >= x_min && x <= x_max) && (y >= y_min && y <= y_max) && (z >= z_min && z <= z_max) {
            // eliminate the particles which are out of domain
            // get the index of the particle
            let nx = ((x - x_min) / cell_size) as usize;
            let ny = ((y - y_min) / cell_size) as usize;
            let nz = ((z - z_min) / cell_size) as usize;
            let no_xy_cells = no_x_cells * no_y_cells;

            let idx = nz * no_xy_cells + ny * no_x_cells + nx;


            for neighbour_idx in &[
                Some(idx),
                idx.checked_sub(1),
                idx.checked_add(1),
                idx.checked_sub(self.no_x_cells),
                idx.checked_sub(self.no_x_cells + 1),
                idx.checked_sub(self.no_x_cells - 1),
                idx.checked_add(self.no_x_cells),
                idx.checked_add(self.no_x_cells - 1),
                idx.checked_add(self.no_x_cells + 1),
            ] {
                match neighbour_idx {
                    Some(index) => {
                        if *index < total_no_cells {
                            particle_idx = head[*index];
                            while particle_idx != usize_max_value {
                                neighbours.push(particle_idx);
                                particle_idx = next[particle_idx];
                            }
                        }
                    }
                    _ => {}
                }
            }

            // for the stack of z = +1
            for neighbour_idx in &[
                idx.checked_add(no_xy_cells),
                idx.checked_add(no_xy_cells - 1),
                idx.checked_add(no_xy_cells + 1),
                idx.checked_add(no_xy_cells - no_y_cells),
                idx.checked_add(no_xy_cells - no_y_cells + 1),
                idx.checked_add(no_xy_cells - no_y_cells - 1),
                idx.checked_add(no_xy_cells + no_y_cells),
                idx.checked_add(no_xy_cells + no_y_cells - 1),
                idx.checked_add(no_xy_cells + no_y_cells + 1),
            ] {
                match neighbour_idx {
                    Some(index) => {
                        if *index < total_no_cells {
                            particle_idx = head[*index];
                            while particle_idx != usize_max_value {
                                neighbours.push(particle_idx);
                                particle_idx = next[particle_idx];
                            }
                        }
                    }
                    _ => {}
                }
            }

            // for the stack of z = -1
            for neighbour_idx in &[
                idx.checked_sub(no_xy_cells),
                idx.checked_sub(no_xy_cells - 1),
                idx.checked_sub(no_xy_cells + 1),
                idx.checked_sub(no_xy_cells - no_y_cells),
                idx.checked_sub(no_xy_cells - no_y_cells + 1),
                idx.checked_sub(no_xy_cells - no_y_cells - 1),
                idx.checked_sub(no_xy_cells + no_y_cells),
                idx.checked_sub(no_xy_cells + no_y_cells - 1),
                idx.checked_sub(no_xy_cells + no_y_cells + 1),
            ] {
                match neighbour_idx {
                    Some(index) => {
                        if *index < total_no_cells {
                            particle_idx = head[*index];
                            while particle_idx != usize_max_value {
                                neighbours.push(particle_idx);
                                particle_idx = next[particle_idx];
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        neighbours
    }
}
