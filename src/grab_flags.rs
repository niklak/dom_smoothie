use flagset::flags;

flags! {
    pub enum GrabFlags: u8 {
        StripUnlikelys,
        WeightClasses,
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
