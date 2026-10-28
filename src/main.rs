use data_structures_rust::stack::Stack;



pub fn main() {
  let mut s: Stack<u32> = Stack::new();
  s.push(10);
  s.push(20);
  s.push(30);
  s.push(40);

  println!("{}", s.pop().unwrap());
  println!("{}", s.pop().unwrap());
  println!("{}", s.pop().unwrap());
}