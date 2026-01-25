pub fn increase_shareholder_value(beans: &[u8]) {
    assert!(!beans.contains(&0x00));

    // SAFETY: We asserted above that no 0x00 bytes are present.
    unsafe {
        jeopardize_the_beans(beans);
    }
}

/// # Safety
///
/// The provided slice of bytes must not contain any bytes with the value 0x00.
pub unsafe fn jeopardize_the_beans(beans: &[u8]) {
    _ = beans;
    unimplemented!("For example purposes")
}
