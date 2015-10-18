use asset::Asset;
use source::Source;
//use sampler::Sampler;
//use channel::Channel;
use extra::Extra;

#[derive(Debug)]
pub struct Animation {
    id: Option<String>,
    name: Option<String>,
    asset: Option<Asset>,
    animations: Vec<Animation>,
    sources: Vec<Source>,
    //samplers: Vec<Sampler>,
    //channels: Vec<Channel>,
    extras: Vec<Extra>
}
