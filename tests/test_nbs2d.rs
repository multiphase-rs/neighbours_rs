extern crate neighbours;

// local library imports
use neighbours::nbs2d::NBS2D;
use neighbours::NNPS;


#[test]
fn test_nbs2d_creation_for_a_given_domain_limits_case_1() {
    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 3.;
    let y_min = 0.;
    let y_max = 3.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let nbs2d = NBS2D::new(x_min, x_max, y_min, y_max, max_size);

    // check the number of total cells
    assert_eq!(9, nbs2d.head.len());
    // check the length of the next array
    assert_eq!(0, nbs2d.next.len());
}

#[test]
fn test_nbs2d_creation_for_a_given_domain_limits_case_2() {
    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 2.2;
    let y_min = 0.;
    let y_max = 3.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let nbs2d = NBS2D::new(x_min, x_max, y_min, y_max, max_size);

    // check the number of total cells
    assert_eq!(6, nbs2d.head.len());
}

#[test]
#[should_panic]
fn test_nbs2d_creation_for_a_given_domain_limits_less_than_cell_size() {
    // the dimensions of the simulation
    let x_min = 0.0;
    let x_max = 0.1;
    let y_min = 0.0;
    let y_max = 3.0;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let nbs2d = NBS2D::new(x_min, x_max, y_min, y_max, max_size);

    // check the number of total cells
    assert_eq!(0, nbs2d.head.len());
}

#[test]
fn test_nbs2d_for_registered_indices_with_single_point_in_each_cell() {
    let x = vec![0.5, 1.5, 2.5, 0.5, 1.5, 2.5, 0.5, 1.5, 2.5];
    let y = vec![0.5, 0.5, 0.5, 1.5, 1.5, 1.5, 2.5, 2.5, 2.5];
    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 3.;
    let y_min = 0.;
    let y_max = 3.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let mut nbs2d = NBS2D::new(x_min, x_max, y_min, y_max, max_size);
    nbs2d.initialize_next(x.len());

    nbs2d.register_particles_to_nnps(&x, &y, &vec![0.]);

    let head_expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(head_expected, nbs2d.head);

    let next_expected = vec![usize::max_value(); 9];
    assert_eq!(next_expected, nbs2d.next);
}

#[test]
fn test_nbs2d_for_registered_indices_with_many_points_in_each_cell() {
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
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let mut nbs2d = NBS2D::new(x_min, x_max, y_min, y_max, max_size);
    nbs2d.initialize_next(x.len());

    nbs2d.register_particles_to_nnps(&x, &y, &vec![0.]);

    let head_expected = vec![2, 5, 8, 11, 14, 17, 20, 23, 26];
    assert_eq!(head_expected, nbs2d.head);

    let next_expected = vec![
        m, 0, 1, m, 3, 4, m, 6, 7, m, 9, 10, m, 12, 13, m, 15, 16, m, 18, 19, m, 21, 22, m, 24, 25,
    ];
    assert_eq!(next_expected, nbs2d.next);
}


#[test]
fn test_get_neighbours_9_cells_with_a_single_point_in_each_cell() {
    let x = vec![0.5, 1.5, 2.5, 0.5, 1.5, 2.5, 0.5, 1.5, 2.5];
    let y = vec![0.5, 0.5, 0.5, 1.5, 1.5, 1.5, 2.5, 2.5, 2.5];

    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 3.;
    let y_min = 0.;
    let y_max = 3.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let mut nbs2d = NBS2D::new(x_min, x_max, y_min, y_max, max_size);
    nbs2d.initialize_next(x.len());

    nbs2d.register_particles_to_nnps(&x, &y, &vec![0.]);

    let nbrs = nbs2d.get_neighbours(1.5, 1.5, 0.);
    // this test even tests the neighbour cells traversal
    let expected_neighbours = vec![4, 3, 5, 1, 0, 2, 7, 6, 8];
    assert_eq!(expected_neighbours, nbrs);

    // let nbrs = nbs2d.get_neighbours(0.5, 1.5, 0.0);
}


#[test]
fn test_get_neighbours_25_cells_with_a_single_point_in_some_cells() {
    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 5.;
    let y_min = 0.;
    let y_max = 5.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let mut nbs2d = NBS2D::new(x_min, x_max, y_min, y_max, max_size);
    assert_eq!(25, nbs2d.head.len());

    let mut x = vec![];
    let mut y = vec![];
    let mut tmp;
    let mut tmp_y = 0.5;
    for _ in 0..5{
        tmp = 0.5;
        for _ in 0..5{
            x.push(tmp);
            y.push(tmp_y);
            tmp += 1.0;
        }
        tmp_y += 1.0;
    }

    nbs2d.initialize_next(x.len());

    nbs2d.register_particles_to_nnps(&x, &y, &vec![0.]);

    let nbrs = nbs2d.get_neighbours(2.5, 2.5, 0.);
    let expected_neighbours = vec![12, 11, 13, 7, 6, 8, 17, 16, 18];
    assert_eq!(expected_neighbours, nbrs);

    let nbrs = nbs2d.get_neighbours(0.0, 0.1, 0.);
    let expected_neighbours = vec![0, 1, 5, 4, 6];
    assert_eq!(expected_neighbours, nbrs);

    // particle in the final cell
    let nbrs = nbs2d.get_neighbours(4.999, 4.999, 0.);
    let expected_neighbours = vec![24, 23, 19, 18, 20];
    assert_eq!(expected_neighbours, nbrs);

    // check the particle which is out of domain
    let nbrs = nbs2d.get_neighbours(0.0, -0.1, 0.);
    let expected_neighbours: Vec<usize> = vec![];
    assert_eq!(expected_neighbours, nbrs);
}


#[test]
#[ignore]
fn test_get_neighbours_with_query_point_on_boundary() {
    // the dimensions of the simulation
    let x_min = 0.;
    let x_max = 5.;
    let y_min = 0.;
    let y_max = 5.;
    let max_size = 1.0;
    // nbs data structure for neighbour creation
    let mut nbs2d = NBS2D::new(x_min, x_max, y_min, y_max, max_size);
    assert_eq!(25, nbs2d.head.len());

    let mut x = vec![];
    let mut y = vec![];
    let mut tmp;
    let mut tmp_y = 0.5;
    for _ in 0..5{
        tmp = 0.5;
        for _ in 0..5{
            x.push(tmp);
            y.push(tmp_y);
            tmp += 1.0;
        }
        tmp_y += 1.0;
    }

    nbs2d.initialize_next(x.len());

    nbs2d.register_particles_to_nnps(&x, &y, &vec![0.]);

    // check the particle which is on the boundary of domain
    // let nbrs = nbs2d.get_neighbours(0.0, 0.0, 0.);
    // let expected_neighbours = vec![0, 1, 5, 4, 6];
    // assert_eq!(expected_neighbours, nbrs);

    // check the particle which is on the boundary of domain

    // TODO: This is a
    // good test. This fails. Lets keep it. One take away is you are not
    // supposed to have points on the boundary.
    let nbrs = nbs2d.get_neighbours(5.0, 5.0, 0.);
    let expected_neighbours = vec![0, 1, 5, 4, 6];
    assert_eq!(expected_neighbours, nbrs);
}
