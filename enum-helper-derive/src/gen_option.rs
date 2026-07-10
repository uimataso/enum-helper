#[derive(Clone)]
pub struct GenOption {
    pub ident: syn::Ident,
    pub vis: syn::Visibility,
}

#[derive(Clone)]
pub struct DefaultGenOption {
    pub enabled: bool,
    pub ident: syn::Ident,
    pub vis: syn::Visibility,
}
