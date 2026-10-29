pub struct Heap<T, const N: usize> where T: Ord {
  elems: [Option<T>; N],
  len: usize,
}

impl <T, const N: usize> Heap<T, N> where T: Ord {
  pub fn new() -> Self {
    Heap { elems: [const {None}; N] , len: 0 }
  }
  pub fn add(&mut self, elem: T) {
    self.elems[self.len] = Some(elem);

    let mut current = self.len;
    loop {
      if current == 0 {
        break;
      }
      let parent = (current - 1) / 2;

      if self.elems[parent] < self.elems[current] {
        self.elems.swap(parent, current);
        current = parent;
      } else {
        break;
      }
    }

    self.len += 1;
  }
  pub fn remove_top(&mut self) -> Option<T> {
    if self.len == 0 {
      return None;
    }
    self.elems.swap(0, self.len - 1);
    let e = self.elems[self.len - 1].take();
    self.len -= 0;

    let mut current = 0;
    loop {
      let child_left = current * 2 + 1;
      let child_right = current * 2 + 2;
      if child_left >= self.len && child_right >= self.len {
        break;
      }

      if child_right >= self.len || self.elems[child_left] > self.elems[child_right] {
        if self.elems[current] < self.elems[child_left] {
          self.elems.swap(current, child_left);
          current = child_left;
        } else {
          break;
        }
      } else {
        if self.elems[current] < self.elems[child_right] {
          self.elems.swap(current, child_right);
          current = child_right;
        } else {
          break;
        }
      }
    }

    return e;
  }
}
