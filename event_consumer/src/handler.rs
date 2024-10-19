mod event;

use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest};
use log::{debug, error};
use crate::line::messaging_api::request::EventObjectRequest;
use serde_json::from_str;
pub async fn handle_post_event(event: ApiGatewayProxyRequest) -> Result<(), lambda_runtime::Error> {
    // parse
    if let Some(body) = event.body {
        let request: serde_json::Result<EventObjectRequest> = from_str(body.as_str());
        debug!("LINE Request body: {:?}", request);
    } else {
        error!("Request Body not found.");
    }

    // todo 署名検証

    // 処理
    Ok(())
}