use syn::LitStr;

#[derive(Debug, darling::FromMeta)]
pub struct RawHandlerArgs {
    #[darling(default)]
    pub search: bool,
    #[darling(default)]
    pub link_resolver: bool,
    #[darling(default)]
    pub source_provider: bool,
    #[darling(default)]
    pub source_resolver: bool,
    pub supports: Vec<LitStr>,
}

impl RawHandlerArgs {
    pub fn validate_type(&self) -> Result<HandlerType, String> {
        let mut true_fields = Vec::new();

        if self.search {
            true_fields.push(HandlerType::Search);
        }
        if self.link_resolver {
            true_fields.push(HandlerType::LinkResolver);
        }
        if self.source_provider {
            true_fields.push(HandlerType::SourceProvider);
        }
        if self.source_resolver {
            true_fields.push(HandlerType::SourceResolver);
        }

        match true_fields.len() {
            1 => Ok(true_fields[0]),
            _ => Err(
                "exactly one of `search`, `link_resolver`, `source_provider` or `source_resolver` must be set"
                    .to_string(),
            ),
        }
    }
}

pub struct HandlerArgs {
    pub handler_type: HandlerType,
    pub supports: Vec<LitStr>,
}

impl TryFrom<RawHandlerArgs> for HandlerArgs {
    type Error = String;

    fn try_from(raw: RawHandlerArgs) -> Result<Self, Self::Error> {
        let handler_type = raw.validate_type()?;

        Ok(HandlerArgs {
            handler_type,
            supports: raw.supports,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum HandlerType {
    Search,
    LinkResolver,
    SourceProvider,
    SourceResolver,
}
