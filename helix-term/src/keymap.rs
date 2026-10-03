pub mod default;
pub mod enzyme;
pub mod macros;

pub use crate::commands::MappableCommand;
pub use default::default;

use arc_swap::{
    access::{DynAccess, DynGuard},
    ArcSwap,
};
use helix_view::{document::Mode, info::Info, input::KeyEvent};
use indexmap::IndexMap;
use macros::key;
use serde::Deserialize;
use std::{
    borrow::Cow,
    collections::{BTreeSet, HashMap},
    ops::{Deref, DerefMut},
    sync::Arc,
};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct KeyTrie {
    name: String,
    #[serde(flatten)]
    map: IndexMap<KeyEvent, KeyTrie>,
    #[serde(skip)]
    pub is_sticky: bool,
}

impl KeyTrie {
    pub fn new(name: &str, map: IndexMap<KeyEvent, KeyTrie>) -> Self {
        Self {
            name: name.to_string(),
            map,
            is_sticky: false,
        }
    }

    /// Merge another Node in. Leaves and subnodes from the other node replace
    /// corresponding keyevent in self, except when both other and self have
    /// subnodes for same key. In that case the merge is recursive.
    pub fn merge(&mut self, other: Self) {
        for (key, trie) in other.map {
            if let Some(KeyTrie::Node(node)) = self.map.get_mut(&key) {
                if let KeyTrie::Node(other_node) = trie {
                    node.merge(other_node);
                    continue;
                }
            }
            self.map.insert(key, trie);
        }
    }

    pub fn infobox(&self) -> Info {
        let mut body = Vec::new();
        for (key, trie) in self.map.iter() {
            let desc = match trie {
                KeyTrie::Node(node) => &node.name,
                KeyTrie::Leaf { desc, .. } | KeyTrie::Sequence { desc, .. } => desc,
                KeyTrie::StickyNode(node) => &node.name,
            };
            body.push((BTreeSet::from([key]), desc));
        }

        let body: Vec<_> = body
            .into_iter()
            .map(|(events, desc)| {
                let events: Vec<String> = events.iter().map(ToString::to_string).collect::<Vec<_>>();
                (events.join(", "), desc)
            })
            .collect();
        Info::new(self.name.clone(), &body)
    }
}

impl Deref for KeyTrie {
    type Target = IndexMap<KeyEvent, KeyTrie>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for KeyTrie {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyTrie {
    MappableCommand(MappableCommand),
    Sequence(Vec<MappableCommand>),
    Node(KeyTrieNode),
    StickyNode(KeyTrieNode),
}
