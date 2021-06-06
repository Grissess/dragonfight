use std::marker::PhantomData;
use std::ops::{BitOr, BitAnd};

macro_rules! flags {
    ($vis:vis $name:ident $contents:tt) => {
        #[repr(u8)]
        $vis enum $name $contents;
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Flags<E> {
    bits: bit_set::BitSet,
    _marker: PhantomData<E>,
}

impl<E: Into<usize>> Flags<E> {
    pub fn new() -> Self {
        Self { bits: bit_set::BitSet::new(), _marker: PhantomData }
    }
}

impl<E: Into<usize>> BitOr<E> for Flags<E> {
    type Output = Self;
    fn bitor(mut self, o: E) -> Self {
        self.bits.insert(o.into());
        self
    }
}

impl<E: Into<usize>> BitAnd<E> for &Flags<E> {
    type Output = bool;
    fn bitand(self, o: E) -> bool {
        self.bits.contains(o.into())
    }
}
