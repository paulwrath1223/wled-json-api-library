use reqwest::Url;


fn main() {
    let url: Url = Url::try_from("http://192.168.1.40/").unwrap();
    let mut wled: Wled = Wled::try_from_url(&url).unwrap();

    println!("new wled: {wled:?}");

    wled.get_cfg_from_wled().unwrap();


    println!("received cfg: {:?}", wled.cfg);

}
