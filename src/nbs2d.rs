use crate::NNPS;

#[derive(Debug, Clone)]
pub struct NBS2D {
    pub head: Vec<usize>,
    pub next: Vec<usize>,
    pub no_x_cells: usize,
    pub no_y_cells: usize,
    pub total_no_cells: usize,
    pub cell_size: f64,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

impl NBS2D {
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64, cell_size: f64) -> NBS2D {
        // check if the domain size is greater than the cell size
        if cell_size > (x_max - x_min) || cell_size > (y_max - y_min) {
            panic!(
                "cell size cannot be greater than the simulation domain, please set your cell size"
            );
        }
        let no_x_cells = ((x_max - x_min) / cell_size) as usize;
        let no_y_cells = ((y_max - y_min) / cell_size) as usize;
        NBS2D {
            head: vec![usize::max_value(); no_x_cells * no_y_cells],
            next: vec![],
            no_x_cells: no_x_cells,
            no_y_cells: no_y_cells,
            total_no_cells: no_x_cells * no_y_cells,
            cell_size: cell_size,
            x_min: x_min,
            x_max: x_max,
            y_min: y_min,
            y_max: y_max,
        }
    }
    pub fn initialize_next(&mut self, no_of_particles: usize) {
        self.next = vec![usize::max_value(); no_of_particles];
    }

    pub fn from_limits_and_no_of_particles(
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        cell_size: f64,
        no_of_particles: usize,
    ) -> NBS2D {
        let mut nbs2d = NBS2D::new(x_min, x_max, y_min, y_max, cell_size);
        nbs2d.initialize_next(no_of_particles);
        nbs2d
    }

    pub fn from_maximum_coordinate(max: f64, cell_size: f64) -> NBS2D {
        let no_x_cells = ((2. * max) / cell_size) as usize;
        let no_y_cells = ((2. * max) / cell_size) as usize;
        NBS2D {
            head: vec![usize::max_value(); no_x_cells * no_y_cells],
            next: vec![],
            no_x_cells: no_x_cells,
            no_y_cells: no_y_cells,
            total_no_cells: no_x_cells * no_y_cells,
            cell_size: cell_size,
            x_min: -max,
            x_max: max,
            y_min: -max,
            y_max: max,
        }
    }

    pub fn from_maximum_and_no_of_particles(
        max: f64,
        cell_size: f64,
        no_of_particles: usize,
    ) -> NBS2D {
        let mut nbs2d = NBS2D::from_maximum_coordinate(max, cell_size);
        nbs2d.initialize_next(no_of_particles);
        nbs2d
    }
}

impl NNPS for NBS2D {
    fn register_particles_to_nnps(&mut self, x: &[f64], y: &[f64], _: &[f64]) {
        let max_value = usize::max_value();

        let x_min = self.x_min;
        let x_max = self.x_max;

        let y_min = self.y_min;
        let y_max = self.y_max;
        let cell_size = self.cell_size;
        let no_x_cells = self.no_x_cells;
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
            if (x[i] >= x_min && x[i] <= x_max) && (y[i] >= y_min && y[i] <= y_max) {
                // eliminate the particles which are out of domain
                // get the index of the particle
                let nx = ((x[i] - x_min) / cell_size) as usize;
                let ny = ((y[i] - y_min) / cell_size) as usize;

                let idx = ny * no_x_cells + nx;

                if idx < total_no_cells {
                    next[i] = head[idx];
                    head[idx] = i;
                }
            }
        }
    }

    fn get_neighbours(&self, x: f64, y: f64, _: f64) -> Vec<usize> {
        let mut neighbours: Vec<usize> = vec![];
        let mut particle_idx;
        let head = &self.head;
        let next = &self.next;
        let total_no_cells = self.total_no_cells;
        let usize_max_value = usize::max_value();

        // check if the particle is in the simulation domain
        if (x >= self.x_min && x <= self.x_max) && (y >= self.y_min && y <= self.y_max) {
            // get the index in the head array
            let nx = ((x - self.x_min) / self.cell_size) as usize;
            let ny = ((y - self.y_min) / self.cell_size) as usize;
            let idx = ny * self.no_x_cells + nx;

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
        }
        neighbours
    }
}
