use proc_macro2::Span;
use quote::ToTokens;
use syn::{Ident, Token, Visibility};

pub trait PathString {
    fn path_string(&self) -> String;
}

#[derive(Clone)]
pub struct PathElement {
    pub ident: Ident,
    pub vis: Visibility,
}

impl std::hash::Hash for PathElement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
        self.vis.to_token_stream().to_string().hash(state);
    }
}

impl Eq for PathElement {}

impl PartialEq for PathElement {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
            && self.vis.to_token_stream().to_string() == other.vis.to_token_stream().to_string()
    }
}

impl PathElement {
    pub fn new(ident: &Ident, visibility: &Visibility) -> Self {
        Self {
            ident: ident.clone(),
            vis: visibility.clone(),
        }
    }

    pub fn public(ident: &Ident) -> Self {
        Self {
            ident: ident.clone(),
            vis: Visibility::Public(Token![pub](Span::call_site())),
        }
    }
}
impl PathString for PathElement {
    fn path_string(&self) -> String {
        format!("({}){}", self.vis.to_token_stream(), self.ident.to_string())
    }
}

impl PathString for Vec<PathElement> {
    fn path_string(&self) -> String {
        self.iter()
            .map(PathElement::path_string)
            .collect::<Vec<_>>()
            .join("::")
    }
}

impl PathString for Vec<Ident> {
    fn path_string(&self) -> String {
        self.iter()
            .map(|segment| segment.to_token_stream().to_string())
            .collect::<Vec<_>>()
            .join("::")
    }
}
