use asset::Asset;
use extra::Extra;
use ::ControlElement;

#[derive(Debug)]
pub struct Controller {
    id: Option<String>,
    name: Option<String>,
    asset: Option<Asset>,
    control_element: ControlElement,
    extra: Vec<Extra>
}
