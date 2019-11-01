#[derive(Debug, Clone)]
pub struct NBS2D {
    pub head: Vec<usize>,
    pub next: Vec<usize>,
    pub no_x_cells: usize,
    pub no_y_cells: usize,
    pub cell_size: f32,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl NBS2D {
    pub fn new(x_min: f32, x_max: f32, y_min: f32, y_max: f32, cell_size: f32) -> NBS2D {
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
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
        cell_size: f32,
        no_of_particles: usize,
    ) {
        let mut nbs2d = NBS2D::new(x_min, x_max, y_min, y_max, cell_size);
        nbs2d.initialize_next(no_of_particles);
    }
}

impl NBS2D {
    pub fn register_particles_to_nbs2d_nnps(&mut self, x: &[f32], y: &[f32]) {
        let max_value = usize::max_value();

        let mut nx;
        let mut ny;
        let x_min = self.x_min;
        let x_max = self.x_max;

        let y_min = self.y_min;
        let y_max = self.y_max;
        let cell_size = self.cell_size;
        let no_x_cells = self.no_x_cells;
        let no_y_cells = self.no_y_cells;

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

        let mut idx;
        for i in 0..x.len() {
            if (x[i] >= x_min && x[i] <= x_max) && (y[i] >= y_min && y[i] <= y_max) {
                // eliminate the particles which are out of domain
                // get the index of the particle
                nx = ((x[i] - x_min) / cell_size) as usize;
                ny = ((y[i] - y_min) / cell_size) as usize;

                idx = ny * no_x_cells + nx;
                next[i] = head[idx];
                head[idx] = i;
            }
        }
    }

    pub fn get_neighbours(&self, x: f32, y: f32) -> Vec<usize> {
        let mut neighbours: Vec<usize> = vec![];
        let mut particle_idx;
        let head = &self.head;
        let next = &self.next;
        let usize_max_value = usize::max_value();

        // check if the particle is in the simulation domain
        if (x >= self.x_min && x <= self.x_max) && (y >= self.y_min && y <= self.y_max) {
            // println!("query point is inside the domain");
            // get the index in the head array
            let nx = ((x - self.x_min) / self.cell_size) as usize;
            let ny = ((y - self.y_min) / self.cell_size) as usize;
            let idx = ny * self.no_x_cells + nx;

            // println!("x index {} y index {} and head index {}", nx, ny, idx);

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
                        particle_idx = head[*index];
                        // println!("particle_idx before while loop {}", particle_idx);
                        while particle_idx != usize_max_value {
                            neighbours.push(particle_idx);
                            particle_idx = next[particle_idx];
                            // println!("particle_idx inside while loop {}", particle_idx);
                        }
                    }
                    _ => {}
                }
            }
        }
        neighbours
    }
}
