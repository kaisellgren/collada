use asset::Asset;
use extra::Extra;
use technique::Technique;

#[derive(Debug)]
pub struct ForceField {
    id: Option<String>,
    name: Option<String>,
    asset: Option<Asset>,
    techniques: Vec<Technique>,
    extras: Vec<Extra>
}
