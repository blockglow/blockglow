use std::iter;

use derive_more::derive::{Deref, DerefMut};
use fast::collections::array::Array;

use super::{Component, Meta};

#[derive(Clone, Ord, Eq, PartialEq, Default, Debug, Deref, DerefMut, Hash)]
pub struct Archetype(Array<Meta, { Self::MAX }>);

impl IntoIterator for Archetype {
    type Item = Meta;

    type IntoIter = fast::collections::array::IntoIter<Self::Item, 256>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialOrd for Archetype {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }

        let supertype = self
            .iter()
            .all(|x| other.iter().find(|y| y.id == x.id).is_some());

        Some(if supertype {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        })
    }
}

impl FromIterator<Meta> for Archetype {
    fn from_iter<I: IntoIterator<Item = Meta>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<Meta> for Archetype {
    fn from(meta: Meta) -> Self {
        Self::from_iter(iter::once(meta))
    }
}

impl Archetype {
    pub const MAX: usize = 256;
    pub fn size(&self) -> usize {
        self.offset_of(self.count())
    }
    pub fn count(&self) -> usize {
        self.len()
    }
    pub(crate) fn offset_of(&self, index: usize) -> usize {
        self.iter()
            .copied()
            .map(|x| x.size)
            .take(index)
            .sum::<usize>()
            .max(1)
    }
    pub(crate) fn merge(&mut self, archetype: Archetype) {
        for meta in archetype {
            if self.iter().find(|x| *x == &meta).is_some() {
                continue;
            }
            self.push(meta);
        }
    }
}
