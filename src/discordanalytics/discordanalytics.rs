use http::header;

pub struct DiscordAnalytics {
  api_token: String,
  headers: http::HeaderMap,
}

impl DiscordAnalytics {
  pub fn new(api_token: String) -> DiscordAnalytics {
    let mut headers = header::HeaderMap::new();
    let mut autorization_string = String::from("Bot ");
    autorization_string.push_str(&api_token);
    headers.insert(
      header::AUTHORIZATION,
      header::HeaderValue::from_str(&autorization_string).unwrap(),
    );
    headers.insert(
      header::CONTENT_TYPE,
      header::HeaderValue::from_static("application/json"),
    );

    DiscordAnalytics {
      api_token,
      headers
    }
  }
}