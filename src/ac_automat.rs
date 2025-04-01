use aho_corasick::{AhoCorasick, AhoCorasickKind};

use once_cell::sync::Lazy;

use crate::glob::{MAYBE_CANDIDATES, UNLIKELY_CANDIDATES};

pub(crate) static AC_UNLIKELY: Lazy<AhoCorasick> = Lazy::new(|| ac_automaton(UNLIKELY_CANDIDATES));
pub(crate) static AC_MAYBE: Lazy<AhoCorasick> = Lazy::new(|| ac_automaton(MAYBE_CANDIDATES));


fn ac_automaton(patterns: &[&str]) -> AhoCorasick {
    AhoCorasick::builder()
        .kind(Some(AhoCorasickKind::ContiguousNFA))
        .build(patterns)
        .unwrap()
}


