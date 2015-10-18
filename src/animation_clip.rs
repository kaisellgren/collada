use std::rc::Rc;
use asset::Asset;
use animation::Animation;
use formula::Formula;
use extra::Extra;

#[derive(Debug)]
pub struct AnimationClip {
    id: Option<String>,
    start: Option<f64>,
    end: Option<f64>,
    name: Option<String>,
    asset: Option<Asset>,
    animations: Vec<Rc<Animation>>,
    formulas: Vec<Rc<Formula>>,
    extras: Vec<Extra>
}
