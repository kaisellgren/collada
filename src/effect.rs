use asset::Asset;
use extra::Extra;
//use annotate::Annotate;
use new_param::NewParam;
//use profile::Profile;

#[derive(Debug)]
pub struct Effect {
    id: String,
    name: Option<String>,
    asset: Option<Asset>,
    //annotate: Vec<Annotate>,
    new_param: Vec<NewParam>,
    //profile: Vec<Profile>,
    extra: Vec<Extra>
}
