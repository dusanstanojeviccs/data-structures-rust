struct Node<T> {
  val: T,
  prev: Option<Box<Node<T>>>,
}

pub struct Queue<T> {
  last: Option<Box<Node<T>>>,
  len: usize,
}

impl <T> Queue<T> {
  pub fn new() -> Self {
    Queue{last: None, len: 0}
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn push(&mut self, elem: T) {
    let new_node = Node{val: elem, prev: self.last.take()};
    self.len += 1;
    self.last = Some(Box::new(new_node))
  }

  pub fn pop(&mut self) -> Option<T> {
    let top_node_option = self.last.take();

    if top_node_option.is_some() {
      let mut top_node = top_node_option.unwrap();
      self.last = top_node.prev.take();
      self.len -= 1;
      return Some(top_node.val);
    }

    None
  }

  pub fn poll(&self) -> Option<&T> {
    if self.last.is_some() {
      return self.last.as_ref().map(|node| &node.val)
    }

    None
  }
}