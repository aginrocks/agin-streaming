use plugin_sdk::handler;
use plugin_sdk::{
    context::Context,
    handler::HandlerError,
    plugin::{SearchRequest, SearchResponse},
};

use crate::State;

#[handler(search, supports("example-provider"))]
pub async fn info(
    ctx: Context<State>,
    request: SearchRequest,
) -> Result<SearchResponse, HandlerError> {
    todo!()
}
