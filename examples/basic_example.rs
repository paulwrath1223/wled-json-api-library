use std::time::Duration;
use reqwest::Url;
use wled_json_api_library::wled::Wled;
use wled_json_api_library::structures::{state::State, cfg::Cfg, cfg::cfg_def::Def};

fn main() {

    // create the URL
    let url: Url = Url::try_from("http://192.168.1.40/").unwrap();

    // create the WLED connection
    let mut wled: Wled = Wled::try_from_url(&url).unwrap();
    println!("new wled: {wled:?}");

    // turn off the WLED
    {
        // put the desired change in the internal state data member
        wled.state = Some(State {
            on: Some(true),
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

        // flush and print the server response
        let response = wled.flush_state().unwrap();
        println!("turning the thing off {:?}", response.text());
    }


    // fill internal cfg with result from WLED
    wled.get_cfg_from_wled().unwrap();

    // get the field defining the power on boot default behaviour
    let turn_on_after_boot = wled.cfg.unwrap().def.unwrap().on.unwrap();
    // print it
    println!("received cfg, turn on after boot: {:?}", turn_on_after_boot);


    // put the desired change into the config data member
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

    // print the response.
    let response = wled.flush_config().unwrap();
    println!("toggling: {:?}", response.text());

    // wait for WLED to finish making this change.
    // Around 100 milliseconds should be enough on good hardware,
    // but this is especially slow because it has to read and write from the internal filesystem
    // where the config file is stored
    std::thread::sleep(Duration::from_millis(80));


    // get and print the new state from the server
    wled.get_cfg_from_wled().unwrap();
    let turn_on_after_boot = wled.cfg.unwrap().def.unwrap().on.unwrap();

    println!("received cfg, turn on after boot: {:?}", turn_on_after_boot);

}
