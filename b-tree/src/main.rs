
#[derive(Debug)]
struct BTree {
    is_root: bool,
    max_nums: usize,
    numbers: Vec<i32>,
    inferior_nodes: Vec<BTree>,
}

// fn get_middle_num_idx(vector: &Vec<i32>) -> usize{


// }

impl BTree {
    fn add_num_to_tree(&mut self, num: i32) {
        if self.numbers.len() == self.max_nums {
            if self.inferior_nodes.len() == 0 { //if there are no inferior nodes
                println!("a");
                self.numbers.push(num);
                self.numbers.sort(); //this will be quick usually

                println!("numbers: {:?}", self.numbers);

                let lower_numbers = &self.numbers[..(self.numbers.len()-1)/2].to_vec();
                println!("lower number's vec: {lower_numbers:?}");

                let middle_number = self.numbers[self.numbers.len()/2].clone();
                println!("middle number: {middle_number:?}");

                let higher_numbers = &self.numbers[self.numbers.len()/2+1..].to_vec();
                println!("higher number's vec: {higher_numbers:?}");


                self.inferior_nodes.push(BTree{is_root: false, max_nums: self.max_nums, numbers: lower_numbers.clone(), inferior_nodes: Vec::with_capacity(self.max_nums+1)});
                self.inferior_nodes.push(BTree{is_root: false, max_nums: self.max_nums, numbers: higher_numbers.clone(), inferior_nodes: Vec::with_capacity(self.max_nums+1)});
                self.numbers.clear();
                self.numbers.push(middle_number);
                
            }
            else { //if there are inferior nodes, those have to be traveled as well (recursion should happen)

            }
            
        }
        else if self.numbers.len() < self.max_nums {
            self.numbers.push(num);    
        }
        else {
            panic!("Array exceeding limit upon adding number to the tree? What?");
        }
    }
    fn new(max_nums: usize, numbers: Vec<i32>) -> Self {
        let mut root_tree = BTree {
            is_root: true,
            max_nums: max_nums,
            numbers: numbers.iter().take(max_nums).cloned().collect::<Vec<i32>>(),
            inferior_nodes: Vec::with_capacity(max_nums+1)
        };
        if numbers.len() > max_nums {
            for num in &numbers[max_nums..]{
                println!("loop ran");
                root_tree.add_num_to_tree(*num);
            }
            
            return root_tree;
        }
        else {
            return root_tree;
        }

    }
}



fn main() {
    //max nums should be an even number for now
    let mut my_tree = BTree::new(6, vec![1, 2, 3, 4, 5, 6, 7]);
    println!("Btree currently: {my_tree:?}");

    let mut tree2 = BTree::new(4, vec![1, 2, 6, 9]);
    tree2.add_num_to_tree(3);
    println!("Btree currently: {tree2:?}");
}
