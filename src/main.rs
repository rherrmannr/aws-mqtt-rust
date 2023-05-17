pub use lambda_http::aws_lambda_events::{serde::Deserialize, serde::Serialize, serde_json};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}
#[derive(Serialize, Deserialize)]
struct Payload {
    message: String,
}

async fn func(value: LambdaEvent<Value>) -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_iotdataplane::Client::new(&config);
    let result: Result<Payload, _> = serde_json::from_str(value.payload.to_string().as_str());
    match result {
        Ok(payload) => {
            if payload.message == "Hello from AWS IoT console" {
                let greeting = Payload {
                    message: "Hello from AWS Lambda".into(),
                };
                let answer = aws_sdk_iotdataplane::types::Blob::new(
                    serde_json::to_string(&greeting).unwrap(),
                );
                client
                    .publish()
                    .topic("example/topic")
                    .qos(1)
                    .payload(answer)
                    .send()
                    .await?;
            }
        }
        Err(_) => {}
    }
    Ok(())
}
