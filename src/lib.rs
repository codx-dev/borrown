#![no_std]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::Deref;

/// Borrowed or owned, simplified for no-std.
pub enum Borrown<'a, T> {
    /// Shared reference for `T`
    Borrowed(&'a T),
    /// Owned `T`
    Owned(T),
}

impl<'a, T> Borrown<'a, T> {
    /// Evaluate if the instance is borrowed
    pub const fn is_borrowed(&self) -> bool {
        matches!(self, Self::Borrowed(_))
    }

    /// Evaluate if the instance is owned
    pub const fn is_owned(&self) -> bool {
        matches!(self, Self::Owned(_))
    }
}

impl<'a, T> Borrown<'a, T>
where
    T: Clone,
{
    /// Return owned `T`. Will clone if is borrowed.
    pub fn into_owned(self) -> T {
        match self {
            Self::Borrowed(b) => b.clone(),
            Self::Owned(o) => o,
        }
    }
}

impl<'a, T> Clone for Borrown<'a, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(b) => Self::Borrowed(b),
            Self::Owned(o) => Self::Owned(o.clone()),
        }
    }
}

impl<'a, T, D> AsRef<D> for Borrown<'a, T>
where
    T: Borrow<D>,
    &'a T: Borrow<D>,
{
    fn as_ref(&self) -> &D {
        match self {
            Self::Borrowed(b) => b.borrow(),
            Self::Owned(o) => o.borrow(),
        }
    }
}

impl<'a, T> AsMut<T> for Borrown<'a, T>
where
    T: Clone,
{
    fn as_mut(&mut self) -> &mut T {
        match *self {
            Self::Borrowed(b) => {
                *self = Self::Owned(b.clone());

                match *self {
                    Self::Borrowed(..) => unreachable!(),
                    Self::Owned(ref mut owned) => owned,
                }
            }

            Self::Owned(ref mut owned) => owned,
        }
    }
}

impl<'a, T> Borrow<T> for Borrown<'a, T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<'a, T, D> Deref for Borrown<'a, T>
where
    T: Deref<Target = D>,
{
    type Target = D;

    fn deref(&self) -> &Self::Target {
        self.as_ref().deref()
    }
}

impl<'a, T> fmt::Debug for Borrown<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a, T> fmt::Display for Borrown<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a, T> Default for Borrown<'a, T>
where
    T: Default,
{
    fn default() -> Self {
        Self::Owned(Default::default())
    }
}

impl<'a, T> From<T> for Borrown<'a, T> {
    fn from(t: T) -> Self {
        Self::Owned(t)
    }
}

impl<'a, T> From<&'a T> for Borrown<'a, T> {
    fn from(t: &'a T) -> Self {
        Self::Borrowed(t)
    }
}

impl<'a, T> PartialEq for Borrown<'a, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<'a, T> Eq for Borrown<'a, T> where T: Eq {}

impl<'a, T> Hash for Borrown<'a, T>
where
    T: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.as_ref().hash(state)
    }
}

impl<'a, T> PartialOrd for Borrown<'a, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<'a, T> Ord for Borrown<'a, T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}
