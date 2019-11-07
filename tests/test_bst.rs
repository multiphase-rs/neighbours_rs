extern crate neighbours;

// local library imports
use neighbours::bst::Node;


#[test]
fn test_bst_insert(){
    let mut x = Node { val: 1, l: None, r: None };
    x.insert(100);
    x.insert(3);
    x.insert(6);
    println!("{:?}", x);
}
