use evercrypt_tiny_sys::EverCrypt_AutoConfig2_init;

/// Tests if the library was successfully built and linked by calling `EverCrypt_AutoConfig2_init`
#[test]
fn test_autoconfig() {
    unsafe { EverCrypt_AutoConfig2_init() };
}
