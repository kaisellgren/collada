use asset::Asset;
use technique::Technique;

#[derive(Debug)]
pub struct Extra {
    id: Option<String>,
    name: Option<String>,
    typ: Option<String>,
    asset: Option<Asset>,
    techniques: Vec<Technique>
}
