use asset::Asset;
use extra::Extra;
//use optics::Optics;
//use imager::Imager;

#[derive(Debug)]
pub struct Camera {
    id: Option<String>,
    name: Option<String>,
    asset: Option<Asset>,
    //optics: Optics,
    //imager: Option<Imager>,
    extra: Vec<Extra>
}
