use reqwest;
use serde_json::Value;

#[derive(Debug, Clone)]
pub (crate) struct HttpClient {
    pub (crate) host: String,
    pub (crate) token: String,
    pub (crate) application_id: String,
}

impl HttpClient {
    pub fn new(host: &str, application_id: String, token: String) -> HttpClient {
        HttpClient {
            host: host.to_string(),
            application_id,
            token
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

        let mut client = reqwest::Client::new()
        .request(
            reqwest::Method::from_bytes(method.as_bytes()).unwrap(), 
            format!("https://{}{}", self.host, path).as_str()
        );
        client = match body {
            Some(body) => client.json(&body),
            None =>  client
        };

        if use_auth {
            client = client.header("Authorization", format!("Bot {}", self.token).as_str());
        }
        client = match content_type {
            Some(content_type) => client.header("Content-Type", content_type),
            None => client.header("Content-Type", "application/json")
        };
        client.send().await.unwrap().json().await.unwrap()
    }

    pub (crate) async fn register_commands(&self, commands: Value) -> Value {
        self.clone().request(
            "PUT", 
            format!("/applications/{}/commands", self.application_id).as_str(), 
            Some(commands), 
            None, true
        ).await
    }

    pub (crate) async fn post_interaction_callback(&self, token: String, id: String, payload: Value) -> Value {
        self.clone().request(
            "POST", 
            format!("/interactions/{}/{}/callback",id,token).as_str(), 
            Some(payload), 
            None, 
            false
        ).await
    }

}