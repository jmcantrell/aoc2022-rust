use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign};

use anyhow::anyhow;

pub const NUM_RESOURCES: usize = 4;

pub type ResourceContainer<T> = [T; NUM_RESOURCES];

pub const RESOURCES: ResourceContainer<Resource> = [
    Resource::Ore,
    Resource::Clay,
    Resource::Obsidian,
    Resource::Geode,
];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    fn index(&self) -> usize {
        match self {
            Self::Ore => 0,
            Self::Clay => 1,
            Self::Obsidian => 2,
            Self::Geode => 3,
        }
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ore => "ore",
                Self::Clay => "clay",
                Self::Obsidian => "obsidian",
                Self::Geode => "geode",
            }
        )?;

        Ok(())
    }
}

impl TryFrom<&str> for Resource {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ore" => Ok(Self::Ore),
            "clay" => Ok(Self::Clay),
            "obsidian" => Ok(Self::Obsidian),
            "geode" => Ok(Self::Geode),
            _ => Err(anyhow!("invalid resource: {:?}", s)),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ResourceMap<T>(ResourceContainer<T>);

impl<T> ResourceMap<T> {
    pub fn iter(&self) -> impl Iterator<Item = (&Resource, &T)> {
        RESOURCES.iter().zip(self.0.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Resource, &mut T)> {
        RESOURCES.iter().zip(self.0.iter_mut())
    }

    pub fn ore(&self) -> &T {
        self.index(&Resource::Ore)
    }

    pub fn ore_mut(&mut self) -> &mut T {
        self.index_mut(&Resource::Ore)
    }

    pub fn clay(&self) -> &T {
        self.index(&Resource::Clay)
    }

    pub fn clay_mut(&mut self) -> &mut T {
        self.index_mut(&Resource::Clay)
    }

    pub fn obsidian(&self) -> &T {
        self.index(&Resource::Obsidian)
    }

    pub fn obsidian_mut(&mut self) -> &mut T {
        self.index_mut(&Resource::Obsidian)
    }

    pub fn geode(&self) -> &T {
        self.index(&Resource::Geode)
    }

    pub fn geode_mut(&mut self) -> &mut T {
        self.index_mut(&Resource::Geode)
    }
}

impl<T: PartialEq> PartialEq for ResourceMap<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: Eq> Eq for ResourceMap<T> {}

impl<T: std::hash::Hash> std::hash::Hash for ResourceMap<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Default> ResourceMap<T> {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<T> From<ResourceContainer<T>> for ResourceMap<T> {
    fn from(values: ResourceContainer<T>) -> Self {
        Self(values)
    }
}

impl<T> Into<ResourceContainer<T>> for ResourceMap<T> {
    fn into(self) -> ResourceContainer<T> {
        self.0
    }
}

impl<T> IntoIterator for ResourceMap<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<Self::Item, NUM_RESOURCES>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Default> From<HashMap<Resource, T>> for ResourceMap<T> {
    fn from(mut map: HashMap<Resource, T>) -> Self {
        RESOURCES
            .map(|resource| map.remove(&resource).unwrap_or_default())
            .into()
    }
}

impl<T> From<ResourceMap<T>> for HashMap<Resource, T> {
    fn from(map: ResourceMap<T>) -> HashMap<Resource, T> {
        RESOURCES.into_iter().zip(map.0.into_iter()).collect()
    }
}

impl<T> Index<&Resource> for ResourceMap<T> {
    type Output = T;

    fn index(&self, resource: &Resource) -> &Self::Output {
        &self.0[resource.index()]
    }
}

impl<T> IndexMut<&Resource> for ResourceMap<T> {
    fn index_mut(&mut self, resource: &Resource) -> &mut Self::Output {
        &mut self.0[resource.index()]
    }
}

pub type ResourceCount = u16;

pub type ResourceTally = ResourceMap<ResourceCount>;

impl ResourceTally {
    pub fn iter_non_zero(&self) -> impl Iterator<Item = (&Resource, &ResourceCount)> {
        self.iter().filter(|(_, &n)| n > 0)
    }
}

impl Copy for ResourceTally {}

impl Ord for ResourceTally {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for ResourceTally {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Add<Self> for ResourceTally {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        std::array::from_fn(|i| self.0[i] + other.0[i]).into()
    }
}

impl AddAssign<Self> for ResourceTally {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Add<Resource> for ResourceTally {
    type Output = Self;

    fn add(mut self, resource: Resource) -> Self {
        self[&resource] += 1;
        self
    }
}

impl Sub<Self> for ResourceTally {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        std::array::from_fn(|i| self.0[i] - other.0[i]).into()
    }
}

impl SubAssign<Self> for ResourceTally {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<Self> for ResourceTally {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        std::array::from_fn(|i| self.0[i] * other.0[i]).into()
    }
}

impl Mul<ResourceCount> for ResourceTally {
    type Output = Self;

    fn mul(self, other: ResourceCount) -> Self {
        self.0.map(|value| value * other).into()
    }
}
