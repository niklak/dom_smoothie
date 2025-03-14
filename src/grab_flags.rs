use flagset::flags;

flags! {
    /// Flags for the grab function, controlling different heuristics for content extraction.
    pub enum GrabFlags: u8 {
        /// Removes elements that are unlikely to be part of the main content.
        StripUnlikelys,
        /// Considers element class and id attributes when calculating content scores.
        WeightClasses,
        /// Applies additional content cleaning after identifying the main content.
        CleanConditionally,
    }
}

#[cfg(test)]
mod tests {
    use flagset::FlagSet;

    use super::*;

    #[test]
    fn test_grab_flags() {
        let mut flags: FlagSet<GrabFlags> = FlagSet::full();
        assert!(flags.contains(GrabFlags::StripUnlikelys));
        flags -= GrabFlags::StripUnlikelys;
        assert!(!flags.contains(GrabFlags::StripUnlikelys));
    }
}
