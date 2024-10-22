use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

pub struct Mem<K, T, const M: usize> {
    pub data: [T; M],
    key_type: PhantomData<K>,
}

impl<K, T: Default + Copy, const M: usize> Default for Mem<K, T, M> {
    fn default() -> Self {
        Self {
            data: [T::default(); M],
            key_type: PhantomData,
        }
    }
}

impl<K, T: Default + Copy, const M: usize> Mem<K, T, M> {
    pub fn clear(&mut self) {
        self.data.fill(T::default())
    }
}

impl<K: Into<usize>, T, const M: usize> Index<K> for Mem<K, T, M> {
    type Output = T;
    fn index(&self, index: K) -> &Self::Output {
        &self.data[index.into()]
    }
}

impl<K: Into<usize>, T, const M: usize> IndexMut<K> for Mem<K, T, M> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.data[index.into()]
    }
}
