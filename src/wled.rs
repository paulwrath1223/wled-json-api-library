use std::time::Duration;

use reqwest;
use reqwest::{Client, Url};

use crate::errors::WledJsonApiError;
use crate::structures::cfg::Cfg;
use crate::structures::effects::Effects;
use crate::structures::info::Info;
use crate::structures::live::Live;
use crate::structures::net::Net;
use crate::structures::nodes::Nodes;
use crate::structures::palettes::Palettes;
use crate::structures::state::State;

struct Wled {
    pub effects: Option<Effects>,
    pub palettes: Option<Palettes>,
    pub state: Option<State>,
    pub info: Option<Info>,
    pub cfg: Option<Cfg>,
    pub live: Option<Live>,
    pub nodes: Option<Nodes>,
    pub net: Option<Net>,
    pub client: Client, // should probably be private in most cases, but fuck you
    pub url: Url,
}

impl Wled{


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
                    effects: None,
                    palettes: None,
                    state: None,
                    info: None,
                    cfg: None,
                    live: None,
                    nodes: None,
                    net: None,
                    client: temp_client,
                    url: temp_url,
                })
            }
            Ok(o) => {Err(WledJsonApiError::HttpError(o.status()))}
            Err(e) => {Err(WledJsonApiError::ReqwestError(e))}
        }
    }

    pub async fn flush_state(&self) -> Result<(), WledJsonApiError> {

        match &self.state{
            Some(s) => {
                let packet: String = s.try_into()?;

                let mut temp_url = self.url.clone();
                temp_url.set_path("json/state");

                self.client.post(temp_url).body(packet).send()
                    .await?;

                Ok(())
            }
            None => Err(WledJsonApiError::FlushNone)
        }
    }

    pub async fn get_effects_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/eff");
        self.effects = Some(Effects::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send().await?
                .text().await?
        )?);
        Ok(())
    }

    pub async fn get_state_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/state");
        self.state = Some(State::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send().await?
                .text().await?
        )?);
        Ok(())
    }

    pub async fn get_cfg_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/cfg");
        self.cfg = Some(Cfg::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send().await?
                .text().await?
        )?);
        Ok(())
    }

    pub async fn get_net_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/net");
        self.net = Some(Net::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send().await?
                .text().await?
        )?);
        Ok(())
    }

    pub async fn get_nodes_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/nodes");
        self.nodes = Some(Nodes::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send().await?
                .text().await?
        )?);
        Ok(())
    }

    pub async fn get_palettes_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/pal");
        self.palettes = Some(Palettes::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send().await?
                .text().await?
        )?);
        Ok(())
    }

    pub async fn get_live_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/live");
        self.live = Some(Live::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send().await?
                .text().await?
        )?);
        Ok(())
    }
}