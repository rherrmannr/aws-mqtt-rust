pub use lambda_http::aws_lambda_events::{serde::Deserialize, serde_json};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(_: LambdaEvent<Value>) -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_iotdataplane::Client::new(&config);
    let message = aws_sdk_iotdataplane::types::Blob::new("input");
    client
        .publish()
        .topic("output")
        .qos(1)
        .payload(message)
        .send()
        .await?;
    Ok(())
}
