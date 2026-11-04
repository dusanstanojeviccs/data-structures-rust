use data_structures_rust::deque::Deque;



pub fn main() {
  let mut d = Deque::new();
  d.enqueue_top(10);
  println!("{}", d.dequeue_top().unwrap());


  d.enqueue_top(10);
  d.enqueue_top(20);
  println!("{}", d.dequeue_top().unwrap());
  println!("{}", d.dequeue_top().unwrap());


  d.enqueue_top(10);
  d.enqueue_top(20);
  println!("{}", d.dequeue_bottom().unwrap());
  println!("{}", d.dequeue_bottom().unwrap());


  d.enqueue_bottom(10);
  println!("{}", d.dequeue_bottom().unwrap());

  println!("pre {:?}", d);
  d.enqueue_bottom(10);
  println!("mid {:?}", d);
  d.enqueue_bottom(20);
  println!("{:?}", d);
  println!("{}", d.dequeue_bottom().unwrap());
  println!("{}", d.dequeue_bottom().unwrap());


  d.enqueue_bottom(10);
  d.enqueue_bottom(20);
  println!("{}", d.dequeue_top().unwrap());
  println!("{}", d.dequeue_top().unwrap());

}