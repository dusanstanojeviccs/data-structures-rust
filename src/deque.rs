struct Node<T> {
  prev: Option<*mut Node<T>>,
  next: Option<*mut Node<T>>,
  value: T,
}

#[derive(Debug)]
pub struct Deque<T> {
  top: Option<*mut Node<T>>,
  bottom: Option<*mut Node<T>>,
  len: usize,
}

impl <T> Deque <T>  {
  pub fn new() -> Self {
    Deque { top: None, bottom: None, len: 0 }
  }
  pub fn enqueue_top(&mut self, elem: T) {
    unsafe {
      let mut e = Box::into_raw(Box::new(Node{prev: None, next: None, value: elem}));
      if self.len == 0 {
        // we're putting on top and bottom
        self.top = Some(e);
        self.bottom = Some(e);
      } else {
        let old_top = self.top.take().unwrap();
        (*e).prev = Some(old_top);
        (*old_top).next = Some(e);
        self.top = Some(e);
      }
      self.len += 1;
    }
  }
  pub fn enqueue_bottom(&mut self, elem: T) {
    unsafe {
      let mut e = Box::into_raw(Box::new(Node{prev: None, next: None, value: elem}));
      if self.len == 0 {
        // we're putting on top and bottom
        self.top = Some(e);
        self.bottom = Some(e);
      } else {
        let old_bottom = self.bottom.take().unwrap();
        (*e).next = Some(old_bottom);
        (*old_bottom).prev = Some(e);
        self.bottom = Some(e);
      }
      self.len += 1;
    }
  }
  pub fn dequeue_top(&mut self) -> Option<T> {
    if self.len == 0 {
      return None;
    }
    unsafe {
      if self.len == 1 {
        self.bottom = None;
      }
      let top = Box::from_raw(self.top.take().unwrap());
      if let Some(p) = top.prev {
        (*p).next = None;
        self.top = Some(p);
      }
      self.len -= 1;
      return Some(top.value);
    }
  }
  pub fn dequeue_bottom(&mut self) -> Option<T> {
    if self.len == 0 {
      return None;
    }
    unsafe {
      if self.len == 1 {
        self.top = None;
      }
      let bottom = Box::from_raw(self.bottom.take().unwrap());
      if let Some(p) = bottom.next {
        (*p).prev = None;
        self.bottom = Some(p);
      }

      self.len -= 1;
      return Some(bottom.value);
    }
  }
}