#[inline]
pub fn bool_to_str(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}
