use asset::Asset;
use extra::Extra;
use ::ControlCategory;

#[derive(Debug)]
pub struct ArticulatedSystem {
    id: Option<String>,
    name: Option<String>,
    asset: Option<Asset>,
    control_category: ControlCategory,
    extra: Vec<Extra>
}
