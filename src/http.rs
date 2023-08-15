use reqwest;
use serde_json::Value;

#[derive(Debug, Clone)]
pub (crate) struct HttpClient {
    pub (crate) host: String,
    pub (crate) token: String,
    pub (crate) application_id: String,
    client : reqwest::Client
}

impl HttpClient {
    pub fn new(host: &str, application_id: String, token: String) -> HttpClient {
        HttpClient {
            host: host.to_string(),
            application_id,
            token,
            client: reqwest::Client::new()
        }
    }
    async fn request(
        self, 
        method: &str, 
        path: &str, 
        body: Option<Value>,
        content_type: Option<&str>, 
        use_auth: bool
    ) -> Value {
        let mut builder = self.client.request(
            reqwest::Method::from_bytes(method.as_bytes()).unwrap(),
            format!("https://{}{}", self.host, path)
        );
        if use_auth {
            builder = builder.header("Authorization", format!("Bot {}", self.token));
        }
        let builder = match content_type {
            Some(content_type) => builder.header("Content-Type", content_type),
            None => builder.header("Content-Type", "application/json")
        };
        let builder = match body {
            Some(body) => builder.body(body.to_string()),
            None => builder
        };
        let response = builder.send().await.unwrap();
        return response.json().await.unwrap();
    }

    pub (crate) async fn register_commands(&self, commands: Value) -> Value {
        self.clone().request(
            "PUT", 
            format!("/applications/{}/commands", self.application_id).as_str(), 
            Some(commands), 
            None, true
        ).await
    }

}