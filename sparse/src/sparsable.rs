use super::*;
use auto_impl::auto_impl;
use std::collections::*;
use std::ffi::CString;

#[auto_impl(&mut, Box)]
pub trait Sparsable {
    fn sparse_init(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError>;

    fn sparse_updt(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        self.sparse_init(state, metadata, depth)
    }

    fn check_depth(depth: u32) -> Result<(), SparseError> {
        match depth < MAX_SPARSE_DEPTH {
            true => Ok(()),
            false => Err(SparseError::CyclicRef),
        }
    }
}

macro_rules! impl_sparsable_nothing {
    ($x:ident) => {
        impl Sparsable for $x {
            fn sparse_init(
                &mut self,
                _state: &mut SparseState,
                _metadata: &SparseMetadata,
                _depth: u32,
            ) -> Result<(), SparseError> {
                Ok(())
            }
        }
    };
}

impl<'a> Sparsable for &'a str {
    fn sparse_init(
        &mut self,
        _state: &mut SparseState,
        _metadata: &SparseMetadata,
        _depth: u32,
    ) -> Result<(), SparseError> {
        Ok(())
    }
}

impl<'a> Sparsable for &'a [u8] {
    fn sparse_init(
        &mut self,
        _state: &mut SparseState,
        _metadata: &SparseMetadata,
        _depth: u32,
    ) -> Result<(), SparseError> {
        Ok(())
    }
}

impl<K, V> Sparsable for HashMap<K, V>
where
    V: Sparsable,
{
    fn sparse_init(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        let ndepth = depth + 1;
        for i in self.values_mut() {
            i.sparse_init(state, metadata, ndepth)?;
        }
        Ok(())
    }
}

macro_rules! impl_sparsable_iter {
    ($x:ident) => {
        impl<T> Sparsable for $x<T>
        where
            T: Sparsable,
        {
            fn sparse_init(
                &mut self,
                state: &mut SparseState,
                metadata: &SparseMetadata,
                depth: u32,
            ) -> Result<(), SparseError> {
                let ndepth = depth + 1;
                for i in self.iter_mut() {
                    i.sparse_init(state, metadata, ndepth)?;
                }
                Ok(())
            }
        }
    };
}

impl_sparsable_nothing!(bool);
impl_sparsable_nothing!(i8);
impl_sparsable_nothing!(i16);
impl_sparsable_nothing!(i32);
impl_sparsable_nothing!(i64);
impl_sparsable_nothing!(isize);
impl_sparsable_nothing!(u8);
impl_sparsable_nothing!(u16);
impl_sparsable_nothing!(u32);
impl_sparsable_nothing!(u64);
impl_sparsable_nothing!(i128);
impl_sparsable_nothing!(usize);
impl_sparsable_nothing!(f32);
impl_sparsable_nothing!(f64);
impl_sparsable_nothing!(char);
impl_sparsable_nothing!(String);
impl_sparsable_nothing!(CString);
impl_sparsable_iter!(Vec);
impl_sparsable_iter!(VecDeque);
impl_sparsable_iter!(LinkedList);
