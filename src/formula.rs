use new_param::NewParam;
use technique::Technique;
use ::ValueOrParam;

#[derive(Debug)]
pub struct Formula {
    id: Option<String>,
    name: Option<String>,
    sid: String,
    new_params: Vec<NewParam>,
    target: ValueOrParam,
    technique_common: String,
    technique: Vec<Technique>
}
