// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
pub struct SlidingVec<T> {
    data: Vec<T>,
    top: usize,
}

/*
SLIDING VECTOR
Useful for classifying a single vector as two vector partitions:

[ A, B | C, D ]
        ^ this is "top"

On pushing:
[ A, B | C, D, E ]

On popping:
[ A, B, C | D, E ]
           ^ top slides forward
*/

impl<T> SlidingVec<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            top: 0,
        }
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data[self.top..].iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data[self.top..].iter()
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }

    pub fn pop(&mut self) {
        self.top += 1;
    }

    pub fn pop_n(&mut self, n: usize) {
        self.top += n;
    }

    pub fn get_right(&self, id: usize) -> Option<&T> {
        self.data.get(self.top + id)
    }

    pub fn get_mut_right(&mut self, id: usize) -> Option<&mut T> {
        self.data.get_mut(self.top + id)
    }

    pub fn get_left(&self, id: usize) -> Option<&T> {
        if id >= self.top {
            None
        } else {
            self.data.get(id)
        }
    }

    pub fn get_mut_left(&mut self, id: usize) -> Option<&mut T> {
        if id >= self.top {
            None
        } else {
            self.data.get_mut(id)
        }
    }

    // Applies closure to sliding windows on the right partition
    pub fn window_right<R, F: FnMut(&mut [T]) -> R>(&mut self, n: usize, mut closure: F) {
        let len = self.data.len() - self.top;
        let no_groups = (len + n - 1) / n;

        for i in 0..no_groups {
            let start = self.top + i * n;
            let end = (start + n).min(self.data.len());
            let slice = &mut self.data[start..end];
            closure(slice);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data[self.top..].is_empty()
    }
}

impl<T> IntoIterator for SlidingVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(mut self) -> std::vec::IntoIter<T> {
        self.data.drain(0..self.top);
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a SlidingVec<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data[self.top..].iter()
    }
}

impl<'a, T> IntoIterator for &'a mut SlidingVec<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data[self.top..].iter_mut()
    }
}

impl<T> FromIterator<T> for SlidingVec<T> {
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        Self {
            data: iter.into_iter().collect(),
            top: 0,
        }
    }
}
