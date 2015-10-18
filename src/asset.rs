use chrono::datetime::*;
use chrono::offset::utc::UTC;
use geographic_location::GeographicLocation;
use measurement::Measurement;
use extra::Extra;

#[derive(Debug)]
pub struct Asset {
    //contributors: Vec<Contributor>,
    coverage: Option<GeographicLocation>,
    created: DateTime<UTC>,
    modified: DateTime<UTC>,
    keywords: String,
    revision: Option<String>,
    subject: Option<String>,
    title: Option<String>,
    unit: Measurement,
    extra: Vec<Extra>
}
