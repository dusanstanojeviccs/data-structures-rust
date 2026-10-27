use data_structures_rust::ringbuffer::RingBuffer;



pub fn main() {
  let mut rb: RingBuffer<u32, 3> = RingBuffer::new();
  rb.add(10);
  rb.add(20);
  rb.add(30);
  rb.add(40);

  rb.consume_all().iter().for_each(|x| println!("{}", *x));
}