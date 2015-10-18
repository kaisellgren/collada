extern crate chrono;
extern crate xml;
extern crate url;

pub mod altitude;
pub mod animation;
pub mod animation_clip;
pub mod articulated_system;
pub mod asset;
pub mod camera;
pub mod contributor;
pub mod controller;
pub mod effect;
pub mod extra;
pub mod force_field;
pub mod formula;
pub mod geographic_location;
pub mod measurement;
pub mod new_param;
pub mod param;
pub mod source;
pub mod technique;

use std::io;
use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use xml::reader::EventReader;
use xml::reader::events::*;
use chrono::datetime::*;
use url::Url;

use param::Param;
use animation_clip::AnimationClip;
use animation::Animation;
use articulated_system::ArticulatedSystem;
use asset::Asset;
use camera::Camera;
use controller::Controller;
use effect::Effect;
use extra::Extra;
use force_field::ForceField;
use formula::Formula;

#[derive(Debug)]
pub enum ControlCategory { Kinematics, Motion }

#[derive(Debug)]
pub enum ControlElement { Skin, Morph }

#[derive(Debug)]
pub enum UpAxis { X_UP, Y_UP, Z_UP }

#[derive(Debug)]
pub enum DataArray {
    Bool(Vec<bool>),
    Float(Vec<f64>),
    IdRef(Vec<String>),
    Int(Vec<u64>),
    Name(Vec<String>),
    SidRef(Vec<String>),
    Token(Vec<String>)
}

#[derive(Debug)]
pub enum ValueOrParam {
    Bool(bool),
    Int(u64),
    Float(f64),
    Float2((f64, f64)),
    SidRef(String),
    Param(Param)
}

#[derive(Debug)]
pub struct Collada {
    version: String,
    asset: Asset,
    animation_clips: Vec<AnimationClip>,
    animations: Vec<Animation>,
    articulated_systems: Vec<ArticulatedSystem>,
    cameras: Vec<Camera>,
    controllers: Vec<Controller>,
    effects: Vec<Effect>,
    force_fields: Vec<ForceField>,
    formulas: Vec<Formula>,
    /*geometries: Vec<Geometry>,
    images: Vec<Image>,
    joints: Vec<Joint>,
    kinematics_models: Vec<KinematicsModel>,
    kinematics_scenes: Vec<KinematicsScene>,
    lights: Vec<Light>,
    materials: Vec<Material>,
    nodes: Vec<Node>,
    physics_materials: Vec<PhysicsMaterial>,
    physics_scenes: Vec<PhysicsScene>,
    visual_scenes: Vec<VisualScene>,
    scene: Option<Scene>,*/
    extras: Vec<Extra>
}

/*pub fn parse<R: Read>(input: R) -> Model {
    let file = BufReader::new(input);

    let mut parser = EventReader::new(file);
    let mut depth = 0;

    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {
                depth += 1
            }
            XmlEvent::EndElement { name } => {
                depth -= 1
            }
            XmlEvent::Error(e) => {
                println!("Error: {}", e);
                break
            }
            _ => ()
        }
    }

    Model { foo: 5 }
}*/
