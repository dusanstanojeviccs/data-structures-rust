use data_structures_rust::heap::Heap;



pub fn main() {
  let mut s: Heap<u32, 3> = Heap::new();
  s.add(10);
  s.add(30);
  s.add(20);

  println!("{}", s.remove_top().unwrap());
  println!("{}", s.remove_top().unwrap());
  println!("{}", s.remove_top().unwrap());
}