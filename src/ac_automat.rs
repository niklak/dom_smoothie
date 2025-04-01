use aho_corasick::{AhoCorasick, AhoCorasickKind};
use once_cell::sync::Lazy;

use crate::glob::{CLASSES_NEGATIVE, CLASSES_POSITIVE, MAYBE_CANDIDATES, UNLIKELY_CANDIDATES};

pub(crate) static AC_UNLIKELY: Lazy<AhoCorasick> = Lazy::new(|| ac_automaton(UNLIKELY_CANDIDATES));
pub(crate) static AC_MAYBE: Lazy<AhoCorasick> = Lazy::new(|| ac_automaton(MAYBE_CANDIDATES));
pub(crate) static AC_CLASSES_NEGATIVE: Lazy<AhoCorasick> =
    Lazy::new(|| ac_automaton(CLASSES_NEGATIVE));
pub(crate) static AC_CLASSES_POSITIVE: Lazy<AhoCorasick> =
    Lazy::new(|| ac_automaton(CLASSES_POSITIVE));

fn ac_automaton(patterns: &[&str]) -> AhoCorasick {
    AhoCorasick::builder()
        .kind(Some(AhoCorasickKind::ContiguousNFA))
        .build(patterns)
        .unwrap()
}
