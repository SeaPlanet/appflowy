use crate::{errors::NetworkError, future::ResultFuture};
use bytes::Bytes;
use protobuf::{Message, ProtobufError};
use reqwest::{Client, Error, Response};
use std::{
    convert::{TryFrom, TryInto},
    time::Duration,
};
use tokio::sync::{oneshot, oneshot::error::RecvError};

pub async fn http_post<T1, T2>(url: &str, data: T1) -> ResultFuture<T2, NetworkError>
where
    T1: TryInto<Bytes, Error = ProtobufError> + Send + Sync + 'static,
    T2: TryFrom<Bytes, Error = ProtobufError> + Send + Sync + 'static,
{
    let url = url.to_owned();
    ResultFuture::new(async move { post(url, data).await })
}

pub async fn post<T1, T2>(url: String, data: T1) -> Result<T2, NetworkError>
where
    T1: TryInto<Bytes, Error = ProtobufError>,
    T2: TryFrom<Bytes, Error = ProtobufError>,
{
    let request_bytes: Bytes = data.try_into()?;
    let (tx, rx) = oneshot::channel::<Result<Response, _>>();

    tokio::spawn(async move {
        let client = default_client();
        let response = client.post(&url).body(request_bytes).send().await;
        tx.send(response);
    });

    match rx.await {
        Ok(response) => {
            let response = response?;
            let response_bytes = response.bytes().await?;
            let data = T2::try_from(response_bytes)?;
            Ok(data)
        },
        Err(e) => {
            unimplemented!()
        },
    }
}

async fn parse_response<T>(response: Response) -> Result<T, NetworkError>
where
    T: Message,
{
    let bytes = response.bytes().await?;
    parse_bytes(bytes)
}

fn parse_bytes<T>(bytes: Bytes) -> Result<T, NetworkError>
where
    T: Message,
{
    match Message::parse_from_bytes(&bytes) {
        Ok(data) => Ok(data),
        Err(e) => {
            log::error!(
                "Parse bytes for {:?} failed: {}",
                std::any::type_name::<T>(),
                e
            );
            Err(e.into())
        },
    }
}

fn default_client() -> Client {
    let result = reqwest::Client::builder()
        .connect_timeout(Duration::from_millis(500))
        .timeout(Duration::from_secs(5))
        .build();

    match result {
        Ok(client) => client,
        Err(e) => {
            log::error!("Create reqwest client failed: {}", e);
            reqwest::Client::new()
        },
    }
}
