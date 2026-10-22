use data_structures_rust::linkedlist::LinkedList;



pub fn main() {
  let mut ll: LinkedList<u32> = LinkedList::new();
  ll.add(10);
  ll.add(20);
  ll.add(30);
  ll.print();
  println!("------");
  ll.remove_at_index(2);
  ll.print();
}