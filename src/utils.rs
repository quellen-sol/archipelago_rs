pub fn is_important(flags: i32) -> bool {
    flags & 0b01 == 0b01
}
