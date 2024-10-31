use flagset::{flags, FlagSet};

flags! {
    enum GrabFlags: u8 {
        STRIP_UNLIKELYS,
        WEIGHT_CLASSES,
        CLEAN_CONDITIONALLY,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_grab_flags() {
        let mut flags =
            GrabFlags::CLEAN_CONDITIONALLY | GrabFlags::STRIP_UNLIKELYS | GrabFlags::WEIGHT_CLASSES;
        assert!(flags.contains(GrabFlags::STRIP_UNLIKELYS));
        flags -= GrabFlags::STRIP_UNLIKELYS;
        assert!(!flags.contains(GrabFlags::STRIP_UNLIKELYS));
    }
}
