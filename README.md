# wled-json-api-library

easy way to control WLED with their [JSON API](https://kno.wled.ge/interfaces/json-api/).

## Documentation
This library has a pretty sizable amount of documentation for what each field means, 
so even if you are making your own library or can't use Rust, 
this is the best spot I can find to figure out what various feilds mean.
Most are not on the JSON API page, and most of the rest isn't even commented in the WLED source code.

If you are the kind soul who wants to put this documentation of the WLED-docs, and can't read rust, here are some pointers:

 - An ```Option``` is an enumeration that can contain a value (```Some(x)```) or nothing (```None```)
 - Example feild below:
```rust
    /// On/Off state of the light                        // The documentation for the feild
    #[serde(skip_serializing_if = "Option::is_none")]    // Says not to convert to text when sending the root object if it is "None"
    #[serde(default = "none_function")]                  // Says to set this to None if it can't find this feild in the input text.
    pub on: Option<bool>,                                // The field. in this case "on" is the key, and the data type is a bool
// If the field has a line that says "serde(rename = "something")"
// it is likely because the name of the field in WLED is a reserved rust keyword,
// so it has to be somthing else and the actual key string is in the "rename" line
```

## Compatibility
I made and tested this with WLED 14.0, but it's meant to support as many builds and past versions as possible. Future versions may or may not be added, but the feilds already present should still work. 


## Streaming colors
While there **is** a way to stream colors with the JSON API, it sucks and it slow. If this is something you want to do, use the DDP protocol. [rust ddp protocol library](https://github.com/coral/ddp-rs)
I decided not to implement this in this library but if you wish to add it, the info on the [WLED documentation](https://kno.wled.ge/interfaces/json-api/#per-segment-individual-led-control) has that feature documented accurately.


## Example
``` rust
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
```
