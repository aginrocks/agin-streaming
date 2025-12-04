use syn::LitStr;

#[derive(Debug, darling::FromMeta)]
pub struct HandlerArgs {
    #[darling(rename = "")]
    pub r#type: HandlerType,
    pub providers: Vec<LitStr>,
}

#[derive(Debug, darling::FromMeta)]
#[darling(rename_all = "snake_case")]
pub enum HandlerType {
    Search,
    LinkResolver,
    SourceProvider,
}
