
pub struct RingBuffer<T, const N: usize> {
  elems: [Option<T>; N],
  first: usize,
  len: usize,
}

impl <T, const N: usize> RingBuffer<T, N> {
  pub fn new() -> Self {
    RingBuffer { elems: [const { None }; N], first: 0, len: 0 }
  }
  pub fn add(&mut self, elem: T) {
    if self.len == N {
        self.elems[self.first] = Some(elem);
        self.first = (self.first + 1) % N;
    } else {
        let idx = (self.first + self.len) % N;
        self.elems[idx] = Some(elem);
        self.len += 1;
    }
  }
  pub fn consume_all(&mut self) -> Vec<T> {
    let mut to_ret: Vec<T> = Vec::with_capacity(self.len);

    for i in 0..self.len {
      to_ret.push(self.elems[(i + 1 + self.first) % N].take().unwrap())
    }

    self.elems = [const { None }; N];
    self.first = 0;
    self.len = 0;

    to_ret
  }
}