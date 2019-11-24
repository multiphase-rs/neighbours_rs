extern crate neighbours;

// local library imports
use neighbours::nbs3d::NBS3D;
use neighbours::NNPS;

#[test]
fn test_nbs3d_creation_for_a_given_domain_limits_case_1() {
    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 3.;
    let y_min = 0.;
    let y_max = 3.;
    let z_min = 0.;
    let z_max = 1.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let nbs3d = NBS3D::new(x_min, x_max, y_min, y_max, z_min, z_max, max_size);

    // check the number of total cells
    assert_eq!(9, nbs3d.head.len());
    // check the length of the next array
    assert_eq!(0, nbs3d.next.len());
}

#[test]
fn test_nbs3d_creation_for_a_given_domain_limits_case_2() {
    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 2.2;
    let y_min = 0.;
    let y_max = 3.;
    let z_min = 0.;
    let z_max = 1.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let nbs3d = NBS3D::new(x_min, x_max, y_min, y_max, z_min, z_max, max_size);

    // check the number of total cells
    assert_eq!(6, nbs3d.head.len());
}

#[test]
fn test_nbs3d_creation_for_a_given_domain_limits_case_3() {
    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 3.;
    let y_min = 0.;
    let y_max = 3.;
    // this case is different from above two, here the z dimension is varied so
    // that it can have some cells in z direction
    let z_min = 0.;
    let z_max = 2.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let nbs3d = NBS3D::new(x_min, x_max, y_min, y_max, z_min, z_max, max_size);

    // check the number of total cells
    assert_eq!(18, nbs3d.head.len());
    // check the length of the next array
    assert_eq!(0, nbs3d.next.len());
}

#[test]
fn test_nbs3d_creation_for_a_given_domain_limits_case_4() {
    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 2.2;
    let y_min = 0.;
    let y_max = 3.;
    // this case is different from above two, here the z dimension is varied so
    // that it can have some cells in z direction
    let z_min = 0.;
    let z_max = 3.2;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let nbs3d = NBS3D::new(x_min, x_max, y_min, y_max, z_min, z_max, max_size);

    // check the number of total cells
    assert_eq!(18, nbs3d.head.len());
}

#[test]
#[should_panic]
fn test_nbs3d_creation_for_a_given_domain_limits_less_than_cell_size() {
    // the dimensions of the simulation
    let x_min = 0.0;
    let x_max = 1.;
    let y_min = 0.0;
    let y_max = 3.0;
    let z_min = 0.;
    let z_max = 0.1;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let nbs3d = NBS3D::new(x_min, x_max, y_min, y_max, z_min, z_max, max_size);

    // check the number of total cells
    assert_eq!(0, nbs3d.head.len());
}

#[test]
fn test_nbs3d_for_registered_indices_with_single_point_in_each_cell() {
    let x = vec![0.5, 1.5, 2.5, 0.5, 1.5, 2.5, 0.5, 1.5, 2.5];
    let y = vec![0.5, 0.5, 0.5, 1.5, 1.5, 1.5, 2.5, 2.5, 2.5];
    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 3.;
    let y_min = 0.;
    let y_max = 3.;
    let z_min = 0.;
    let z_max = 1.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let mut nbs3d = NBS3D::new(x_min, x_max, y_min, y_max, z_min, z_max, max_size);
    nbs3d.initialize_next(x.len());

    nbs3d.register_particles_to_nnps(&x, &y, &vec![0.; x.len()]);

    let head_expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(head_expected, nbs3d.head);

    let next_expected = vec![usize::max_value(); 9];
    assert_eq!(next_expected, nbs3d.next);
}

#[test]
fn test_nbs3d_for_registered_indices_with_many_points_in_each_cell() {
    let m = usize::max_value();
    let x = vec![
        0.5, 0.6, 0.7, 1.5, 1.6, 1.7, 2.5, 2.6, 2.7, 0.5, 0.6, 0.7, 1.5, 1.6, 1.7, 2.5, 2.6, 2.7,
        0.5, 0.6, 0.7, 1.5, 1.6, 1.7, 2.5, 2.6, 2.7,
    ];
    let y = vec![
        0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5,
        2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5,
    ];
    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 3.;
    let y_min = 0.;
    let y_max = 3.;
    let z_min = 0.;
    let z_max = 1.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let mut nbs3d = NBS3D::new(x_min, x_max, y_min, y_max, z_min, z_max, max_size);
    nbs3d.initialize_next(x.len());

    nbs3d.register_particles_to_nnps(&x, &y, &vec![0.; x.len()]);

    let head_expected = vec![2, 5, 8, 11, 14, 17, 20, 23, 26];
    assert_eq!(head_expected, nbs3d.head);

    let next_expected = vec![
        m, 0, 1, m, 3, 4, m, 6, 7, m, 9, 10, m, 12, 13, m, 15, 16, m, 18, 19, m, 21, 22, m, 24, 25,
    ];
    assert_eq!(next_expected, nbs3d.next);
}

#[test]
fn test_nbs3d_get_neighbours_9_cells_with_a_single_point_in_each_cell() {
    let x = vec![0.5, 1.5, 2.5, 0.5, 1.5, 2.5, 0.5, 1.5, 2.5];
    let y = vec![0.5, 0.5, 0.5, 1.5, 1.5, 1.5, 2.5, 2.5, 2.5];

    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 3.;
    let y_min = 0.;
    let y_max = 3.;
    let z_min = 0.;
    let z_max = 1.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let mut nbs3d = NBS3D::new(x_min, x_max, y_min, y_max, z_min, z_max, max_size);
    nbs3d.initialize_next(x.len());

    nbs3d.register_particles_to_nnps(&x, &y, &vec![0.; x.len()]);

    let nbrs = nbs3d.get_neighbours(1.5, 1.5, 0.);
    // this test even tests the neighbour cells traversal
    let expected_neighbours = vec![4, 3, 5, 1, 0, 2, 7, 6, 8];
    assert_eq!(expected_neighbours, nbrs);
}

#[test]
fn test_nbs3d_get_neighbours_27_cells_with_a_single_point_in_each_cell() {
    let x = vec![
        0.5, 1.5, 2.5, 0.5, 1.5, 2.5, 0.5, 1.5, 2.5, 0.5, 1.5, 2.5, 0.5, 1.5, 2.5, 0.5, 1.5, 2.5,
        0.5, 1.5, 2.5, 0.5, 1.5, 2.5, 0.5, 1.5, 2.5,
    ];
    let y = vec![
        0.5, 0.5, 0.5, 1.5, 1.5, 1.5, 2.5, 2.5, 2.5, 0.5, 0.5, 0.5, 1.5, 1.5, 1.5, 2.5, 2.5, 2.5,
        0.5, 0.5, 0.5, 1.5, 1.5, 1.5, 2.5, 2.5, 2.5,
    ];
    let z = vec![
        -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,
        0.5, 0.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5,
    ];

    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 3.;
    let y_min = 0.;
    let y_max = 3.;
    let z_min = -1.;
    let z_max = 2.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let mut nbs3d = NBS3D::new(x_min, x_max, y_min, y_max, z_min, z_max, max_size);
    nbs3d.initialize_next(x.len());

    // check the number of total cells
    assert_eq!(27, nbs3d.head.len());

    nbs3d.register_particles_to_nnps(&x, &y, &z);

    let nbrs = nbs3d.get_neighbours(1.5, 1.5, 0.5);

    // this test even tests the neighbour cells traversal
    let expected_neighbours = vec![
        13, 12, 14, 10, 9, 11, 16, 15, 17, 22, 21, 23, 19, 20, 18, 25, 24, 26, 4, 5, 3, 7, 6, 8, 1,
        2, 0,
    ];
    assert_eq!(expected_neighbours, nbrs);
}

#[test]
fn test_nbs3d_10_particles_on_x_axis() {
    let x = vec![0., 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.];
    let y = vec![0.; x.len()];
    let z = vec![0.; x.len()];
    let max_coordinate = 2.0;

    let max_size = 2. * 0.05;
    let mut nbs3d = NBS3D::from_maximum_and_no_of_particles(max_coordinate, max_size, x.len());
    nbs3d.register_particles_to_nnps(&x, &y, &z);

    // check the number of total cells
    assert_eq!(64000, nbs3d.head.len());

    nbs3d.register_particles_to_nnps(&x, &y, &z);

    let nbrs = nbs3d.get_neighbours(1.5, 1.5, 0.5);

    // this test even tests the neighbour cells traversal
    // let expected_neighbours = vec![
    //     13, 12, 14, 10, 9, 11, 16, 15, 17, 22, 21, 23, 19, 20, 18, 25, 24, 26, 4, 5, 3, 7, 6, 8, 1,
    //     2, 0,
    // ];
    // assert_eq!(expected_neighbours, nbrs);
}
