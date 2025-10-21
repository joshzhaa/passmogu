use std::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};
use zeroize::Zeroizing;

/// Secret zeroizes the heap allocated u8 slice when dropped. We're only supporting ASCII,
/// and we want to prohibit reallocations. However, it should be safe to use Vec<Secret> b/c
/// reallocations only move the Box pointer around (not leaving secret strings behind).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Secret {
    data: Zeroizing<Box<[u8]>>,
}

impl Secret {
    /// initializes Secret, all zeroes
    pub fn zero(len: usize) -> Self {
        Secret {
            data: Zeroizing::new(vec![0_u8; len].into_boxed_slice()),
        }
    }

    pub fn new(data: Box<[u8]>) -> Self {
        Secret {
            data: Zeroizing::new(data),
        }
    }

    pub fn expose(&self) -> &[u8] {
        &self.data
    }

    pub fn expose_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Creates new, shorter Secret with first len elements from self
    pub fn truncate(&self, len: usize) -> Self {
        let mut shorter = Self::zero(len);
        for i in 0..len {
            shorter[i] = self[i];
        }
        shorter
    }
}

impl<I: SliceIndex<[u8]>> Index<I> for Secret {
    // impl Index<usize> for Secret {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index]
    }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for Secret {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index]
    }
}
