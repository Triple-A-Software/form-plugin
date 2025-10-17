use serde::Deserialize;
use serde_json::json;

const CR_TOKEN_URL: &str = "https://rest.cleverreach.com/oauth/token.php";
const CR_API_URL: &str = "https://rest.cleverreach.com/v3";

#[derive(Clone, Debug)]
pub struct Cleverreach {
    access_token: Option<AccessToken>,
    client: reqwest::Client,
    client_id: Option<String>,
    client_secret: Option<String>,
}

impl Cleverreach {
    pub fn new(client_id: Option<String>, client_secret: Option<String>) -> Self {
        Self {
            access_token: None,
            client: reqwest::Client::new(),
            client_id,
            client_secret,
        }
    }

    pub fn set_client_credentials(
        &mut self,
        client_id: Option<String>,
        client_secret: Option<String>,
    ) {
        self.client_id = client_id;
        self.client_secret = client_secret;
    }

    async fn update_access_token(&mut self) -> anyhow::Result<()> {
        if let (Some(client_id), Some(client_secret)) = (&self.client_id, &self.client_secret) {
            let response = self
                .client
                .post(CR_TOKEN_URL)
                .basic_auth(client_id, Some(client_secret))
                .form(&[("grant_type", "client_credentials")])
                .send()
                .await?;
            let json = response.json::<AccessToken>().await?;
            self.access_token = Some(json);
            Ok(())
        } else {
            Err(anyhow::anyhow!("No client credentials defined"))
        }
    }

    pub async fn create_receiver(&mut self, request: ReceiverRequest) -> anyhow::Result<()> {
        if self.access_token.is_none() {
            self.update_access_token().await?;
        }
        if let Some(access_token) = &self.access_token {
            self.client
                .post(format!(
                    "{CR_API_URL}/groups/{}/receivers",
                    request.group_id
                ))
                .header(
                    "Authorization",
                    format!("Bearer {}", access_token.access_token),
                )
                .json(&json!({
                    "email": request.email,
                    "global_attributes": {
                        "firstname": request.first_name,
                        "lastname": request.last_name,
                        "anrede": request.salutation,
                    },
                    "source": "",
                }))
                .send()
                .await?;
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Unknown error occurred when trying to fetch access token, did you correctly configure the client_id and client_secret?"
            ))
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct AccessToken {
    access_token: String,
    expires_in: usize,
}

pub struct ReceiverRequest {
    pub group_id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub salutation: String,
}
