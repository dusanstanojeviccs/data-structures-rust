struct Node<T> {
  elem: T,
  next: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
  top: Option<Box<Node<T>>>,
  depth: usize,
}

impl <T> Stack<T> {
  pub fn new() -> Self {
    Stack {top: None, depth: 0}
  }

  pub fn push(&mut self, elem: T) {
    let prev_top = self.top.take();

    self.top = Some(Box::new(Node{ elem: elem, next: prev_top}));
    self.depth += 1;
  }
  pub fn pop(&mut self) -> Option<T> {
    if self.depth == 0 {
      return None;
    }
    let mut bn = self.top.take().unwrap();

    self.top = bn.next.take();
    self.depth -= 1;

    return Some(bn.elem);
  }
  pub fn peek(&self) -> Option<&T> {
    if self.depth == 0 {
      return None;
    }
    return self.top.as_ref().map(|x| &x.elem)
  }
}