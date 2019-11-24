pub struct ParticleArray {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub radius: Vec<f64>,
}

impl ParticleArray {
    pub fn new(total_no_particles: usize) -> Self {
        ParticleArray {
            x: vec![0.; total_no_particles],
            y: vec![0.; total_no_particles],
            z: vec![0.; total_no_particles],
            radius: vec![0.; total_no_particles],
        }
    }
    pub fn from_xyz_rad(x: &[f64], y: &[f64], z: &[f64], radius: &[f64]) -> Self {
        let total_no_particles = x.len();
        let mut particles = ParticleArray::new(total_no_particles);
        particles.x = x.to_vec();
        particles.y = y.to_vec();
        particles.z = z.to_vec();
        particles.radius = radius.to_vec();

        return particles;
    }
}

#[macro_export]
macro_rules! write_to_vtk {
    ($dest:ident, $output:expr) => {
        // This is taken from
        // https://lorensen.github.io/VTKExamples/site/VTKFileFormats/#legacy-file-examples
        // let mut filename: String = current_exe().unwrap().to_str().unwrap().to_string();
        // filename.push_str(".vtk");
        let x = &$dest.x;
        let y = &$dest.y;
        let r = &$dest.radius;
        let filename = $output;

        let _ = fs::remove_file(filename);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();

        writeln!(file, "# vtk DataFile Version 3.0").unwrap();
        writeln!(file, "Time some").unwrap();
        writeln!(file, "ASCII\nDATASET UNSTRUCTURED_GRID").unwrap();

        writeln!(file, "POINTS {} float", x.len()).unwrap();
        for i in 0..x.len() {
            writeln!(file, "{:.4} {:.4} 0.0", x[i], y[i]).unwrap();
        }

        writeln!(file, "POINT_DATA {}", x.len()).unwrap();
        writeln!(file, "SCALARS Diameter float 1").unwrap();
        writeln!(file, "LOOKUP_TABLE default").unwrap();
        for i in 0..x.len() {
            writeln!(file, "{:.4}", r[i]).unwrap();
        }
    };
}
