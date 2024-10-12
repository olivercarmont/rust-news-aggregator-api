use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
enum Request {
    Payload { payload: Payload },
    Headers { headers: Headers },
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Payload {
    query: Option<String>,
    news_api_key: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Headers {
    query: Option<String>,
    news_api_key: Option<String>,
}

#[derive(Serialize)]
struct Response {
   req_id: String,
   response: String,
}


async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    println!("Received event: {:?}", event);

    let (query, news_api_key) = match &event.payload {
        Request::Payload { payload } => (
            payload.query.clone().unwrap_or_else(|| "No query found".to_string()),
            payload.news_api_key.clone().unwrap_or_else(|| "No API key found".to_string()),
        ),
        Request::Headers { headers } => (
            headers.query.clone().unwrap_or_else(|| "No query found".to_string()),
            headers.news_api_key.clone().unwrap_or_else(|| "No API key found".to_string()),
        ),
    };

    let news_api_response: Value = ureq::get("https://newsapi.org/v2/everything")
       .query("q", &query)
       .query("apiKey", &news_api_key)
       .call()?
       .into_json()?;


   let articles: Vec<serde_json::Value> = news_api_response["articles"]
       .as_array()
       .unwrap_or(&Vec::new())
       .iter()
       .filter_map(|article| {
           let title = article["title"].as_str()?;
           let source = article["source"]["name"].as_str()?;
           let date = article["publishedAt"].as_str()?;
           Some(serde_json::json!({
               "headline": title,
               "publisher": source,
               "publishedAt": date
           }))
       })
       .collect();


   // Create a JSON response with the articles information
   let resp = Response {
       req_id: event.context.request_id,
       response: serde_json::json!({ "articles": articles }).to_string(),
   };


   Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}