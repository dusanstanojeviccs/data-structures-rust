use data_structures_rust::btree::BTree;



pub fn main() {
  let mut bt: BTree<u32> = BTree::new(2);
  bt.insert(10);
  bt.insert(20);
  bt.insert(30);
  bt.insert(40);
  bt.print();
}