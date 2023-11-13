use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Palettes(Vec<String>);


impl TryFrom<&str> for Palettes{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<Palettes, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}



#[cfg(test)]
mod tests {
    use crate::structures::palettes::Palettes;

    #[test]
    fn it_works() {
        let a: Palettes = Palettes::try_from
            (r#"[
"Default","* Random Cycle","* Color 1","* Colors 1&2","* Color Gradient","* Colors Only","Party","Cloud","Lava","Ocean",
"Forest","Rainbow","Rainbow Bands","Sunset","Rivendell","Breeze","Red & Blue","Yellowout","Analogous","Splash",
"Pastel","Sunset 2","Beach","Vintage","Departure","Landscape","Beech","Sherbet","Hult","Hult 64",
"Drywet","Jul","Grintage","Rewhi","Tertiary","Fire","Icefire","Cyane","Light Pink","Autumn",
"Magenta","Magred","Yelmag","Yelblu","Orange & Teal","Tiamat","April Night","Orangery","C9","Sakura",
"Aurora","Atlantica","C9 2","C9 New","Temperature","Aurora 2","Retro Clown","Candy","Toxy Reaf","Fairy Reaf",
"Semi Blue","Pink Candy","Red Reaf","Aqua Flash","Yelblu Hot","Lite Light","Red Flash","Blink Red","Red Shift","Red Tide",
"Candy2"
]"#).unwrap();
        println!("{:?}", a);

    }
}

