pub enum DefaultValue {
    None,
    Default,
    Value(String),
}

pub fn val(s: &str) -> DefaultValue {
    DefaultValue::Value(s.to_string())
}

const SKIP_DEFAULTS_FOR_FIELDS: &[&str] = &[
    "LoadOp<V>",
    "u32",
    "i32",
    "f32",
    "u64",
    "i64",
    "f64",
    "bool",
];

pub fn get_builder_name(name: &str) -> &str {
    name
}

pub fn get_default_value(
    struct_name: &str,
    field_name: &str,
    field_type: &str,
    has_default: bool,
) -> DefaultValue {
    let mut value = if has_default {
        DefaultValue::Default
    } else {
        DefaultValue::None
    };

    if field_type.starts_with("Option<") || SKIP_DEFAULTS_FOR_FIELDS.contains(&field_type) {
        value = DefaultValue::None;
    };

    if struct_name == "BlendComponent" {
        value = match field_name {
            "src_factor" => val("BlendFactor::One"),
            "dst_factor" => val("BlendFactor::Zero"),
            "operation" => val("BlendOperation::Add"),
            _ => value,
        };
    }

    if struct_name == "Color" {
        value = DefaultValue::Default;
    }

    if struct_name == "DepthBiasState" {
        value = DefaultValue::Default
    }

    if struct_name == "DispatchIndirectArgs" {
        value = DefaultValue::Default
    }

    if struct_name == "DownlevelCapabilities" {
        value = match field_name {
            "flags" => val("DownlevelFlags::all()"),
            "limits" => val("DownlevelLimits::default()"),
            "shader_model" => val("ShaderModel::Sm5"),
            _ => value,
        }
    }

    value
}
