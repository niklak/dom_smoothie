use aho_corasick::AhoCorasick;

use once_cell::sync::Lazy;

use crate::glob::{MAYBE_CANDIDATES, UNLIKELY_CANDIDATES};

pub(crate) static AC_UNLIKELY: Lazy<AhoCorasick> =
    Lazy::new(|| AhoCorasick::new(UNLIKELY_CANDIDATES).unwrap());

pub(crate) static AC_MAYBE: Lazy<AhoCorasick> =
    Lazy::new(|| AhoCorasick::new(MAYBE_CANDIDATES).unwrap());
