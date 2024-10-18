use aws_lambda_events::encodings::Body::Text;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_runtime::{service_fn, LambdaEvent};
use log::{debug};

async fn handler(event: LambdaEvent<ApiGatewayProxyRequest>)
                 -> Result<ApiGatewayProxyResponse, lambda_runtime::Error>
{
    debug!("Event: {:?}", event);

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Text("{\"message\":\"hello lambda world.\"}".to_string())),
        is_base64_encoded: false,
    })
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    env_logger::init();
    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}
