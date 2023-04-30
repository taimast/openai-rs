use std::env;
use std::iter::Map;

use bytes::Bytes;
use futures::{Stream, StreamExt};
use reqwest::{Client, header::HeaderMap, Response};
use std::error::Error;
use crate::{ChatCompletionChunk, OpenAIRequest};

const URL: &str = "https://api.openai.com/v1/chat/completions";

type RequestResponse<T> = Result<T, reqwest::Error>;

pub struct OpenAIClient {
    client: Client,
}

impl OpenAIClient {
    pub fn new(api_key: &str) -> OpenAIClient {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", api_key).parse().unwrap(),
        );
        let client = Client::builder()
            .default_headers(headers)
            .build().unwrap();
        OpenAIClient { client }
    }

    async fn send_request(&self, request: &OpenAIRequest) -> RequestResponse<Response> {
        let response = self
            .client
            .post(URL)
            .json(request)
            .send()
            .await?;
        Ok(response)
    }


    pub async fn new_stream(
        &self,
        request: &OpenAIRequest,
    ) -> RequestResponse<impl Stream<Item=Result<Bytes, reqwest::Error>>> {
        let response = self.send_request(request).await?;
        let stream = response.bytes_stream();
        Ok(stream)
    }

    pub async fn new_map_stream(
        &self,
        request: &OpenAIRequest,
    ) -> RequestResponse<impl Stream<Item=RequestResponse<Vec<ChatCompletionChunk>>>> {
        let stream = self.new_stream(request).await?;
        let stream = stream.map(Self::map_chunk);
        Ok(stream)
    }

    fn map_chunk(chunk: Result<Bytes, reqwest::Error>) -> RequestResponse<Vec<ChatCompletionChunk>> {
        let chunk = chunk?;
        let chunk = ChatCompletionChunk::from_chunk(&chunk).unwrap_or_default();
        Ok(chunk)
    }
}


