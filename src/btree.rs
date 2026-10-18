use std::{fmt::Debug, range};

use crate::queue::Queue;

struct Node<T> {
  keys: Vec<T>,
  children: Vec<Box<Node<T>>>,
}

impl<T> Node<T> {
  pub fn is_leaf(&self) -> bool {
    self.children.is_empty()
  }

  pub fn new(max_elements: usize) -> Self {
    Node{keys: Vec::default(), children: Vec::with_capacity(max_elements)}
  }
}

pub struct BTree<T> {
  min_degree: usize,
  root: Box<Node<T>>,
  len: usize,
}

impl<T> BTree<T> where T: Ord, T: Debug {
  pub fn new(min_degree: usize) -> Self {
    if min_degree < 2 {
      panic!("Bad dev, bad min_degree");
    }
    BTree{len: 0, root: Box::new(Node::new(min_degree * 2 - 1)), min_degree: min_degree}
  }

  pub fn len(&self) -> usize {
    self.len
  }

  fn is_full(&self, node: &Node<T>) -> bool {
    node.keys.len() >= 2*self.min_degree - 1
  }

  fn split(min_degree: usize, to_split: &mut Box<Node<T>>) -> Box<Node<T>> {
    let mut new_root: Box<Node<T>> = Box::new(Node::new(min_degree * 2));
    let mut left: Box<Node<T>> = Box::new(Node::new(min_degree * 2));
    let mut right: Box<Node<T>> = Box::new(Node::new(min_degree * 2));

    let mid_index = to_split.keys.len() / 2;
    let mut mid_elem: Option<T> = None;
    for (i, e) in to_split.keys.drain(..).enumerate() {
      if i < mid_index {
        left.keys.push(e);
      } else if i == mid_index {
        mid_elem = Some(e);
      } else if i > mid_index {
        right.keys.push(e);
      }
    }

    for (i, child) in to_split.children.drain(..).enumerate() {
        if i <= mid_index {
            left.children.push(child);
        } else {
            right.children.push(child);
        }
    }

    new_root.children.push(left);
    new_root.keys.push(mid_elem.unwrap());
    new_root.children.push(right);
    new_root
  }

  pub fn insert(&mut self, elem: T) -> bool {
    if self.is_full(&self.root) {
      let new_node = Self::split(self.min_degree, &mut self.root);
      self.root = new_node;
    }

    let mut to_process: &mut Box<Node<T>> = &mut self.root;

    'outer: loop {
      if to_process.is_leaf() {
        // we have the one we should insert into
        let found = to_process.keys.iter().any(|i| *i == elem);

        if found {
          // we can't insert
          return false;
        }
        let mut index: Option<usize> = None;
        for (i, key) in to_process.keys.iter().enumerate() {
          if *key > elem {
            index = Some(i);
            break;
          }
        }
        if index.is_none() {
          self.len += 1;
          to_process.keys.push(elem);
          return true;
        } else {
          self.len += 1;
          to_process.keys.insert(index.unwrap(), elem);
          return true;
        }
      } else {
        let mut index: Option<usize> = None;
        for (i, key) in to_process.keys.iter().enumerate() {
          if *key == elem {
            return false;
          }
          if *key > elem {
            index = Some(i);
            break;
          }
        }

        if index.is_none() {
          index = Some(to_process.children.len() - 1);
        }

        let i = index.unwrap();
        if to_process.children[i].keys.len() == 2 * self.min_degree - 1 {
          let mut new_node = Self::split(self.min_degree, &mut to_process.children[i]);

          let right = new_node.children.pop().unwrap();   // your new_root.children = [left, right]
          let left = new_node.children.pop().unwrap();
          let median = new_node.keys.pop().unwrap();      // your new_root.keys = [median]

          let mut next_i = i;
          let median_is_elem = elem == median;
          if elem > median {
            next_i = i + 1;
          }
          to_process.children[i] = left;              // replaces the drained child
          to_process.children.insert(i + 1, right);
          to_process.keys.insert(i, median);

          to_process = &mut to_process.children[next_i];

          if median_is_elem {
            return false;
          }
        } else {
          to_process = &mut to_process.children[i];
        }
      }
    }
  }

  pub fn contains(&self, elem: &T) -> bool {
    let mut to_process: &Box<Node<T>> = &self.root;

    'outer: loop {
      if to_process.is_leaf() {
        return to_process.keys.iter().any(|i| i == elem);
      } else {
        for (i, key) in to_process.keys.iter().enumerate() {
          if key == elem {
            return true;
          }
          if key > elem {
            // we take the left hand side
            to_process = to_process.children.get(i).unwrap();
            continue 'outer
          }
        }

        // if we're here nothing got taken we take the last one
        to_process = to_process.children.last().unwrap();
      }
    }
  }

  pub fn print(&self) {
    Self::print_node(&self.root, 0);
  }

  fn print_node(node: &Box<Node<T>>, depth: usize) {
    for i in (0..depth) {
      print!(" ")
    }
    for key in node.keys.iter() {
      print!("{:?},", *key);
    }
    println!("");

    for key in node.children.iter() {
      Self::print_node(key, depth+1);
    }
  }
}