use proc_macro2::TokenStream;
use quote::quote as q;
use syn::Ident;

#[derive(Clone, Debug)]
pub enum DefaultValue {
    Skip,
    Default,
    Value(TokenStream),
}

#[derive(Clone, Debug)]
pub struct FieldConfig {
    pub default_value: DefaultValue,
    pub into: bool,
    pub name: Ident,
    pub ty: TokenStream,
}

#[derive(Clone, Debug)]
pub struct StructConfig {
    pub fields: Vec<FieldConfig>,
    pub name: Ident,
    pub fn_name: Ident,
    pub generics: TokenStream,
}

pub(crate) fn customize_config(struct_config: StructConfig) -> StructConfig {
    let mut struct_config = struct_config.clone();
    struct_config.fields = struct_config
        .fields
        .iter()
        .map(|field| customize_default(&struct_config, field))
        .map(|field| customize_into(&field))
        .collect();

    struct_config
}

pub(crate) fn customize_default(struct_config: &StructConfig, field: &FieldConfig) -> FieldConfig {
    let mut field = field.clone();
    let ty = &field.ty;

    if ty.to_string() == q!(Label<'a>).to_string() {
        field.default_value = DefaultValue::Value(q!(None));
    }

    if ty.to_string() == q!(PowerPreference).to_string() {
        field.default_value = DefaultValue::Default
    }

    if struct_config.name == "Operations" && field.name == "load" {
        field.default_value = DefaultValue::Value(q!(LoadOp::Load))
    }

    if struct_config.name == "RequestAdapterOptionsBase" && field.name == "force_fallback_adapter" {
        field.default_value = DefaultValue::Value(q!(false));
    }

    if struct_config.name == "BlendComponent" {
        field.default_value = match field.name.to_string().as_str() {
            "operation" => DefaultValue::Value(q!(BlendOperation::Add)),
            "src_factor" => DefaultValue::Value(q!(BlendFactor::One)),
            "dst_factor" => DefaultValue::Value(q!(BlendFactor::Zero)),
            _ => field.default_value,
        }
    }

    if struct_config.name == "BufferBinding" && field.name == "offset" {
        field.default_value = DefaultValue::Value(q!(0u64));
    }

    if struct_config.name == "ColorTargetState" && field.name == "write_mask" {
        field.default_value = DefaultValue::Value(q!(ColorWrites::default()));
    }

    if struct_config.name == "CopyExternalImageDestInfo" {
        field.default_value = match field.name.to_string().as_str() {
            "color_space" => DefaultValue::Value(q!(PredefinedColorSpace::Srgb)),
            "premultiplied_alpha" => DefaultValue::Value(q!(false)),
            _ => field.default_value,
        }
    }

    field
}

pub(crate) fn customize_into(field: &FieldConfig) -> FieldConfig {
    let mut field = field.clone();
    let ty = &field.ty;

    if ty.to_string().starts_with("& 'a [") {
        field.into = false;
    }

    field
}
