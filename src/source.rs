use asset::Asset;
use technique::Technique;
use ::DataArray;

#[derive(Debug)]
pub struct Source {
    id: String,
    name: Option<String>,
    asset: Option<Asset>,
    data: Option<DataArray>,
    techniques: Vec<Technique>,
    technique_common: Option<String>
}
