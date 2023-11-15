use std::time::Duration;

use reqwest;
use reqwest::blocking::{Client, ClientBuilder, Response};
use reqwest::Url;

use crate::errors::WledJsonApiError;
use crate::structures::cfg::Cfg;
use crate::structures::effects::Effects;
use crate::structures::info::Info;
use crate::structures::live::Live;
use crate::structures::net::Net;
use crate::structures::nodes::Nodes;
use crate::structures::palettes::Palettes;
use crate::structures::state::State;



#[derive(Debug)]
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
    pub ddp_url: Url,
}

impl Wled{


    pub fn try_from_url(url: &Url) -> Result<Wled, WledJsonApiError> {
        let temp_client: Client = ClientBuilder::new()
            .gzip(true)
            .timeout(Duration::from_millis(5000u64))
            .build()
            .map_err(|e|{WledJsonApiError::ReqwestError(e)})?;
        let mut temp_url: Url = url.clone();
        temp_url.set_path("json/cfg");
        match temp_client
            .get(temp_url.clone())
            .send() {
            Ok(a) if a.status() == reqwest::StatusCode::OK => {
                let temp_url_2 = temp_url.clone();
                temp_url.set_port(Some(4048)).map_err(|_| {WledJsonApiError::UnableToAddPortToURL})?;
                temp_url.set_path("");
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
                    url: temp_url_2,
                    ddp_url: temp_url
                })
            }
            Ok(o) => {Err(WledJsonApiError::HttpError(o.status()))}
            Err(e) => {Err(WledJsonApiError::ReqwestError(e))}
        }

    }


    pub fn flush_state(&self) -> Result<Response, WledJsonApiError> {

        match &self.state{
            Some(s) => {
                let packet: String = s.try_into()?;

                let mut temp_url = self.url.clone();
                temp_url.set_path("json/state");

                Ok(self.client.post(temp_url).body(packet).send()?)
            }
            None => Err(WledJsonApiError::FlushNone)
        }
    }


    /// be careful with this, this library does not stop you from sending invalid and crazy configs.
    /// as long as the feilds make sense it should work, but
    pub fn flush_config(&self) -> Result<Response, WledJsonApiError> {

        match &self.cfg{
            Some(s) => {
                let packet: String = s.try_into()?;

                let mut temp_url = self.url.clone();
                temp_url.set_path("json/cfg");

                Ok(self.client.post(temp_url).body(packet).send()?)
            }
            None => Err(WledJsonApiError::FlushNone)
        }
    }

    pub fn get_effects_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/eff");
        self.effects = Some(Effects::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()?
                .text()?
        )?);
        Ok(())
    }

    pub fn get_state_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/state");
        self.state = Some(State::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()?
                .text()?
        )?);
        Ok(())
    }

    pub fn get_cfg_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/cfg");
        self.cfg = Some(Cfg::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()?
                .text()?
        )?);
        Ok(())
    }

    pub fn get_net_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/net");
        self.net = Some(Net::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()?
                .text()?
        )?);
        Ok(())
    }

    pub fn get_nodes_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/nodes");
        self.nodes = Some(Nodes::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()?
                .text()?
        )?);
        Ok(())
    }

    pub fn get_palettes_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/pal");
        self.palettes = Some(Palettes::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()?
                .text()?
        )?);
        Ok(())
    }

    pub fn get_live_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/live");
        self.live = Some(Live::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()?
                .text()?
        )?);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::structures::cfg::cfg_def::Def;
    use super::*;

    #[test]
    fn it_works() {
        let url: Url = Url::try_from("http://192.168.1.40/").unwrap();
        let mut wled: Wled = Wled::try_from_url(&url).unwrap();

        println!("new wled: {wled:?}");


        // put the desired change in the internal state data member
        wled.state = Some(State{
            on: Some(false),
            bri: None,
            transition: None,
            tt: None,
            ps: None,
            psave: None,
            pl: None,
            nl: None,
            udpn: None,
            v: None,
            rb: None,
            live: None,
            lor: None,
            time: None,
            mainseg: None,
            playlist: None,
            seg: None,
        });

        // flush
        let response = wled.flush_state().unwrap();
        println!("turning the thing off {:?}", response.text());


        // fill internal cfg with result from WLED
        wled.get_cfg_from_wled().unwrap();

        // get the feild defining the power on boot default behaviour
        let config = wled.cfg.clone();
        let turn_on_after_boot = config.unwrap().def.unwrap().on.unwrap();
        // print it
        println!("received cfg, turn on after boot: {:?}", turn_on_after_boot);




        wled.cfg = Some(Cfg{
            rev: None,
            vid: None,
            id: None,
            nw: None,
            eth: None,
            ap: None,
            wifi: None,
            hw: None,
            light: None,
            def: Some(Def{
                ps: None,
                on: Some(!turn_on_after_boot),
                bri: None,
            }),
            if_field: None,
            remote: None,
            ol: None,
            timers: None,
            ota: None,
            dmx: None,
            um: None,
        });

        let response = wled.flush_config().unwrap();

        println!("toggling: {:?}", response.text());


        wled.get_cfg_from_wled().unwrap();

        let config = wled.cfg.clone();

        let turn_on_after_boot = config.unwrap().def.unwrap().on.unwrap();

        println!("received cfg, turn on after boot: {:?}", turn_on_after_boot);
    }
}