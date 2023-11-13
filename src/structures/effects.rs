use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effects(Vec<String>);

impl TryFrom<&str> for Effects{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<Effects, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}


#[cfg(test)]
mod tests {
    use crate::structures::effects::Effects;

    #[test]
    fn it_works() {
        let a: Effects = Effects::try_from
            (r#"["Solid","Blink","Breathe","Wipe","Wipe Random","Random Colors","Sweep","Dynamic","Colorloop","Rainbow","Scan","Scan Dual","Fade","Theater","Theater Rainbow","Running","Saw","Twinkle","Dissolve","Dissolve Rnd","Sparkle","Sparkle Dark","Sparkle+","Strobe","Strobe Rainbow","Strobe Mega","Blink Rainbow","Android","Chase","Chase Random","Chase Rainbow","Chase Flash","Chase Flash Rnd","Rainbow Runner","Colorful","Traffic Light","Sweep Random","Chase 2","Aurora","Stream","Scanner","Lighthouse","Fireworks","Rain","Tetrix","Fire Flicker","Gradient","Loading","Rolling Balls","Fairy","Two Dots","Fairytwinkle","Running Dual","RSVD","Chase 3","Tri Wipe","Tri Fade","Lightning","ICU","Multi Comet","Scanner Dual","Stream 2","Oscillate","Pride 2015","Juggle","Palette","Fire 2012","Colorwaves","Bpm","Fill Noise","Noise 1","Noise 2","Noise 3","Noise 4","Colortwinkles","Lake","Meteor","Meteor Smooth","Railway","Ripple","Twinklefox","Twinklecat","Halloween Eyes","Solid Pattern","Solid Pattern Tri","Spots","Spots Fade","Glitter","Candle","Fireworks Starburst","Fireworks 1D","Bouncing Balls","Sinelon","Sinelon Dual","Sinelon Rainbow","Popcorn","Drip","Plasma","Percent","Ripple Rainbow","Heartbeat","Pacifica","Candle Multi","Solid Glitter","Sunrise","Phased","Twinkleup","Noise Pal","Sine","Phased Noise","Flow","Chunchun","Dancing Shadows","Washing Machine","RSVD","Blends","TV Simulator","Dynamic Smooth","Spaceships","Crazy Bees","Ghost Rider","Blobs","Scrolling Text","Drift Rose","Distortion Waves","Soap","Octopus","Waving Cell","Pixels","Pixelwave","Juggles","Matripix","Gravimeter","Plasmoid","Puddles","Midnoise","Noisemeter","Freqwave","Freqmatrix","GEQ","Waterfall","Freqpixels","RSVD","Noisefire","Puddlepeak","Noisemove","Noise2D","Perlin Move","Ripple Peak","Firenoise","Squared Swirl","RSVD","DNA","Matrix","Metaballs","Freqmap","Gravcenter","Gravcentric","Gravfreq","DJ Light","Funky Plank","RSVD","Pulser","Blurz","Drift","Waverly","Sun Radiation","Colored Bursts","Julia","RSVD","RSVD","RSVD","Game Of Life","Tartan","Polar Lights","Swirl","Lissajous","Frizzles","Plasma Ball","Flow Stripe","Hiphotic","Sindots","DNA Spiral","Black Hole","Wavesins","Rocktaves","Akemi"]"#).unwrap();
        println!("{:?}", a);

    }
}