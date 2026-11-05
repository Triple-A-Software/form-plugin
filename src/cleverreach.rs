use std::time::{Duration, Instant};

use serde::Deserialize;
use serde_json::json;

const CR_TOKEN_URL: &str = "https://rest.cleverreach.com/oauth/token.php";
const CR_API_URL: &str = "https://rest.cleverreach.com/v3";

#[derive(Clone, Debug)]
pub struct Cleverreach {
    access_token: Option<AccessToken>,
    /// when the token has to be renewed, cleverreach tokens expire after `expires_in` seconds
    token_expires_at: Option<Instant>,
    client: reqwest::Client,
    client_id: Option<String>,
    client_secret: Option<String>,
}

impl Cleverreach {
    pub fn new(client_id: Option<String>, client_secret: Option<String>) -> Self {
        Self {
            access_token: None,
            token_expires_at: None,
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
        // the old token belongs to the old credentials
        self.access_token = None;
        self.token_expires_at = None;
    }

    /// Requests a token with the given credentials, for the connection test in the settings.
    pub async fn test_credentials(client_id: &str, client_secret: &str) -> anyhow::Result<()> {
        fetch_access_token(&reqwest::Client::new(), client_id, client_secret).await?;
        Ok(())
    }

    async fn update_access_token(&mut self) -> anyhow::Result<()> {
        if let (Some(client_id), Some(client_secret)) = (&self.client_id, &self.client_secret) {
            let token = fetch_access_token(&self.client, client_id, client_secret).await?;
            // renew a minute early, so a request never runs into the expiry
            self.token_expires_at = Some(
                Instant::now() + Duration::from_secs(token.expires_in as u64).saturating_sub(Duration::from_secs(60)),
            );
            self.access_token = Some(token);
            Ok(())
        } else {
            Err(anyhow::anyhow!("No client credentials defined"))
        }
    }

    pub async fn create_receiver(&mut self, request: ReceiverRequest) -> anyhow::Result<()> {
        let expired = self.token_expires_at.is_none_or(|at| Instant::now() >= at);
        if self.access_token.is_none() || expired {
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
                .await?
                // without this a rejected token or group id would look like a success
                .error_for_status()?;
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Unknown error occurred when trying to fetch access token, did you correctly configure the client_id and client_secret?"
            ))
        }
    }
}

async fn fetch_access_token(
    client: &reqwest::Client,
    client_id: &str,
    client_secret: &str,
) -> anyhow::Result<AccessToken> {
    Ok(client
        .post(CR_TOKEN_URL)
        .basic_auth(client_id, Some(client_secret))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?
        .error_for_status()?
        .json::<AccessToken>()
        .await?)
}

#[derive(Deserialize, Clone, Debug)]
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
