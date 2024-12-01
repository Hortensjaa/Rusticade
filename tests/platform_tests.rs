#[cfg(test)]
mod tests {
    use rusticade::classes::platform::Platform;


    #[test]
    fn test_default_platform() {
        let platform = Platform::default();
        assert_eq!(platform.x, 0.0);
        assert_eq!(platform.y, 0.0);
        assert_eq!(platform.w, 50.0);
        assert_eq!(platform.h, 50.0);

    }

    #[test]
    fn test_new_platform() {
        let platform = Platform::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(platform.x, 10.0);
        assert_eq!(platform.y, 20.0);
        assert_eq!(platform.w, 100.0);
        assert_eq!(platform.h, 50.0);
    }
}
