pub enum DefaultValue {
    None,
    Default,
    Value(String),
}

pub fn get_default_value(
    struct_name: &str,
    field_name: &str,
    field_type: &str,
    has_default: bool,
) -> DefaultValue {
    if has_default && !field_type.starts_with("Option<") {
        DefaultValue::Default
    } else {
        DefaultValue::None
    }
}
