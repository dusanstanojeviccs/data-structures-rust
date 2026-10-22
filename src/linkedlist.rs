use std::fmt::Display;

struct Node<T> {
  val: T,
  next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
  first: Option<Box<Node<T>>>,
  len: usize,
}

impl<T> LinkedList<T> where T: Display {
  pub fn new() -> Self {
    LinkedList { first: None, len: 0 }
  }

  pub fn add(&mut self, elem: T) {
    if self.len == usize::MAX {
      panic!("Oh no, anyways");
    }

    if self.len == 0 {
      self.first = Some(Box::new(Node{val: elem, next: None}));
    } else {
      let mut last = &mut self.first;
      while last.is_some() {
        last = &mut last.as_mut().unwrap().next;
      }

      *last = Some(Box::new(Node{val: elem, next: None}));
    }
    self.len += 1;
  }

  pub fn remove_at_index(&mut self, i: usize) {
    if i >= self.len {
      panic!("Naughty naughty");
    }

    if i == 0 {
      self.first = self.first.take().unwrap().next;
    } else {
      let mut last = &mut self.first;
      for m in 0..(i-1) {
        last = &mut last.as_mut().unwrap().next;
      }
      last.as_mut().unwrap().next = last.as_mut().unwrap().next.take().unwrap().next;
    }
  }

  pub fn print(&self) {
    let mut iter = &self.first;

    while let Some(node) = iter {
      println!("{}", node.val);
      iter = &node.next;
    }

  }
}
