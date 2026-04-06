/// Spotify integration — no DB access, pure service.
pub struct SpotifyService {
    access_token: String,
}

impl SpotifyService {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
        }
    }

    /// Add a track to the user's Spotify playback queue.
    /// `spotify_uri` is of the form `spotify:track:<id>`.
    pub async fn add_to_queue(&self, spotify_uri: &str) -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.spotify.com/v1/me/player/queue")
            .bearer_auth(&self.access_token)
            .query(&[("uri", spotify_uri)])
            .send()
            .await?;

        if res.status().is_success() || res.status().as_u16() == 204 {
            Ok(())
        } else {
            anyhow::bail!(
                "Spotify queue failed: {} {}",
                res.status(),
                res.text().await.unwrap_or_default()
            )
        }
    }
}
