#[derive(Debug)]
pub struct OctTree{
    pub val: usize,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>,
}

impl Node {
    pub fn insert(&mut self, new_val: usize){
        if self.val == new_val {
            return
        }

        let target_node = if new_val < self.val { &mut self.l } else { &mut self.r };

        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = Node { val: new_val, l: None, r: None};
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }

    }
}
