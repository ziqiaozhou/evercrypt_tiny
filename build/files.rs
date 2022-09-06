//! Utilities for working with files

use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, DirEntry},
    ops::Add,
    path::PathBuf,
};

/// The current distribution's "c89-compatible" folder
pub const DIST_C89: &str = "vendored/v0.4.5-dist/c89-compatible";
/// The current distribution's KaRaMeL include folder
pub const DIST_KARAMEL_INCLUDE: &str = "vendored/v0.4.5-dist/kremlin/include";
/// The current distribution's KaRaMeL mini-dist include folder
pub const DIST_KARAMEL_MINIMAL_INCLUDE: &str = "vendored/v0.4.5-dist/kremlin/kremlib/dist/minimal";

/// A pattern
#[derive(Debug, Clone)]
pub enum Pattern<T> {
    /// Whether a string starts with the pattern
    Start(T),
    /// Whether a string ends with the pattern
    End(T),
    /// Whether a string contains the pattern
    Contains(T),
    /// Whether the string is exactly the same as the pattern
    Exact(T),
    /// An aggregated pattern
    Multi(Vec<Pattern<String>>),
}
impl<T> Pattern<T> {
    /// Whether `self` matches the given string
    pub fn matches(&self, string: &str) -> bool
    where
        T: AsRef<str>,
    {
        match self {
            Self::Start(pat) => string.starts_with(pat.as_ref()),
            Self::End(pat) => string.ends_with(pat.as_ref()),
            Self::Contains(pat) => string.contains(pat.as_ref()),
            Self::Exact(pat) => string.eq(pat.as_ref()),
            Self::Multi(pats) => pats.iter().all(|pat| pat.matches(string)),
        }
    }

    /// Canonicalizes `self`
    fn canonical(self) -> Vec<Pattern<String>>
    where
        T: ToString,
    {
        match self {
            Self::Multi(pats) => pats,
            Self::Start(pat) => vec![Pattern::Start(pat.to_string())],
            Self::End(pat) => vec![Pattern::End(pat.to_string())],
            Self::Contains(pat) => vec![Pattern::Contains(pat.to_string())],
            Self::Exact(pat) => vec![Pattern::Exact(pat.to_string())],
        }
    }
}
impl<T> Add for Pattern<T>
where
    T: ToString,
{
    type Output = Pattern<String>;

    fn add(self, other: Self) -> Self::Output {
        let mut pats = self.canonical();
        pats.extend(other.canonical());
        Pattern::Multi(pats)
    }
}

/// A file list
#[derive(Debug, Default)]
pub struct FileList {
    /// The names
    pub names: BTreeSet<String>,
    /// The entries
    pub entries: BTreeMap<String, DirEntry>,
}
impl FileList {
    /// Creates a new empty file list
    pub fn new() -> Self {
        Self { names: BTreeSet::new(), entries: BTreeMap::new() }
    }
    /// Adds all files within `dir` for which `pat` matches
    pub fn add<T>(&mut self, dir: &str, pat: Pattern<T>) -> &mut Self
    where
        T: AsRef<str>,
    {
        // Gather files
        for maybe_entry in fs::read_dir(dir).expect("Failed to open directory") {
            // Unwrap the entry and get the file name
            let entry = maybe_entry.expect("Failed to iterate over directory entry");
            let name = entry.file_name().into_string().expect("Non-UTF-8 file name");

            // Collect the entry if the pattern is satisfied
            if pat.matches(&name) {
                self.names.insert(name.clone());
                self.entries.insert(name, entry);
            }
        }
        self
    }
    /// Removes all files for which `pat` matches
    pub fn remove<T>(&mut self, pat: Pattern<T>) -> &mut Self
    where
        T: AsRef<str>,
    {
        self.names.retain(|name| !pat.matches(name));
        self.entries.retain(|name, _| !pat.matches(name));
        self
    }

    /// Returns an iterator over all item paths
    pub fn paths(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.entries.values().map(|entry| entry.path())
    }
}
