use http::header;

pub struct DiscordAnalytics {
  client: serenity::Client,
  api_token: String,
  headers: http::HeaderMap,
}

impl DiscordAnalytics {
  pub fn new(client: serenity::Client, api_token: String) -> DiscordAnalytics {
    let mut headers = header::HeaderMap::new();
    headers.insert(
      header::AUTHORIZATION,
      header::HeaderValue::from_str(&api_token).unwrap(),
    );
    headers.insert(
      header::CONTENT_TYPE,
      header::HeaderValue::from_static("application/json"),
    );

    DiscordAnalytics {
      client: client,
      api_token: api_token,
      headers
    }
  }
}