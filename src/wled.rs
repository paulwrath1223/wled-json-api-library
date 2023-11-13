
use std::time::{Duration, Instant};
use crate::errors::WledJsonApiError;
use crate::structures::effects::Effects;
use crate::structures::info::Info;
use crate::structures::palettes::Palettes;
use crate::structures::state::State;
use reqwest;
use reqwest::{Client, Url};

struct Wled {
    effects: Effects,
    palettes: Palettes,
    state: State,
    info: Info,
    pub client: Client, // should probably be private in most cases, but fuck you
    pub url: Url,
    pub outgoing_state: State,

}

impl Wled{
    pub fn get_local_effects(&self) -> &Effects {
        &self.effects
    }
    pub fn get_local_palettes(&self) -> &Palettes {
        &self.palettes
    }
    pub fn get_local_state(&self) -> &State {
        &self.state
    }
    pub fn get_local_info(&self) -> &Info {
        &self.info
    }

    pub async fn try_from_url(url: &mut Url) -> Result<Wled, WledJsonApiError> {
        let temp_client: Client = reqwest::ClientBuilder::new()
            .gzip(true)
            .timeout(Duration::from_millis(5000u64))
            .build()
            .map_err(|e|{WledJsonApiError::ReqwestError(e)})?;
        let mut temp_url: Url = url.clone();
        temp_url.set_path("json/cfg");
        match temp_client
            .get(temp_url.clone())
            .send().await {
            Ok(a) if a.status() == reqwest::StatusCode::OK => {
                url.set_port(Some(4048)).map_err(|_| {WledJsonApiError::UnableToAddPortToURL})?;


                Ok(Wled{
                    effects: Default::default(),
                    palettes: Default::default(),
                    state: Default::default(),
                    info: Default::default(),
                    client: Default::default(),
                    url: temp_url,
                    outgoing_state: Default::default(),
                })
            }
            Ok(o) => {Err(WledJsonApiError::HttpError(o.status()))}
            Err(e) => {Err(WledJsonApiError::ReqwestError(e))}
        }
    }

    pub async fn flush_state(&self) -> Result<(), WledJsonApiError> {


        let packet: String = (&self.state).try_into()?;

        let mut temp_url = self.url.clone();
        temp_url.set_path("json/state");

        self.client.post(temp_url).body(packet).send()
            .await?;

        Ok(())
    }

}