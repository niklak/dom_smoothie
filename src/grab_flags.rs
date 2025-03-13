use flagset::{flags, FlagSet};

flags! {
    pub enum GrabFlags: u8 {
        StripUnlikelys,
        WeightClasses,
        CleanConditionally,
    }
}
pub (crate) fn all_grab_flags() -> FlagSet<GrabFlags> {
    GrabFlags::StripUnlikelys | GrabFlags::WeightClasses | GrabFlags::CleanConditionally
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_grab_flags() {
        let mut flags =
            GrabFlags::CleanConditionally | GrabFlags::StripUnlikelys | GrabFlags::WeightClasses;
        assert!(flags.contains(GrabFlags::StripUnlikelys));
        flags -= GrabFlags::StripUnlikelys;
        assert!(!flags.contains(GrabFlags::StripUnlikelys));
    }
}
