// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type Person = Vec<PersonElement>;

#[derive(Serialize, Deserialize)]
pub struct PersonElement {
    #[serde(rename = "_index")]
    pub index: Index,

    #[serde(rename = "_type")]
    pub person_type: Type,

    #[serde(rename = "_id")]
    pub id: String,

    #[serde(rename = "_score")]
    pub score: i64,

    #[serde(rename = "_source")]
    pub source: Source,

    #[serde(rename = "_ignored")]
    pub ignored: Option<Vec<Ignored>>,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "uid")]
    pub uid: i64,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "slug")]
    pub slug: String,

    #[serde(rename = "score")]
    pub score: i64,

    #[serde(rename = "biography")]
    pub biography: Option<BiographyUnion>,

    #[serde(rename = "authorities")]
    pub authorities: Option<Vec<Authority>>,

    #[serde(rename = "categories")]
    pub categories: Vec<Category>,

    #[serde(rename = "descriptions")]
    pub descriptions: Option<Vec<Description>>,

    #[serde(rename = "genders")]
    pub genders: Option<Vec<Category>>,

    #[serde(rename = "locations")]
    pub locations: Option<Vec<SourceLocation>>,

    #[serde(rename = "names")]
    pub names: Vec<Name>,

    #[serde(rename = "occupations")]
    pub occupations: Option<Vec<Occupation>>,

    #[serde(rename = "projects")]
    pub projects: Vec<Project>,

    #[serde(rename = "works")]
    pub works: Option<Vec<SourceWork>>,

    #[serde(rename = "events")]
    pub events: Option<Vec<Event>>,

    #[serde(rename = "performances")]
    pub performances: Option<Vec<SourcePerformance>>,

    #[serde(rename = "corporations")]
    pub corporations: Option<Vec<Corporation>>,

    #[serde(rename = "serials")]
    pub serials: Option<Vec<Serial>>,

    #[serde(rename = "sources")]
    pub sources: Option<Vec<SourceElement>>,
}

#[derive(Serialize, Deserialize)]
pub struct Authority {
    #[serde(rename = "authority")]
    pub authority: i64,
}

#[derive(Serialize, Deserialize)]
pub struct BiographyClass {
    #[serde(rename = "birth")]
    pub birth: Option<Birth>,

    #[serde(rename = "death")]
    pub death: Option<Birth>,
}

#[derive(Serialize, Deserialize)]
pub struct Birth {
    #[serde(rename = "dates")]
    pub dates: Option<Vec<Date>>,

    #[serde(rename = "locations")]
    pub locations: Option<Vec<BirthLocation>>,
}

#[derive(Serialize, Deserialize)]
pub struct Date {
    #[serde(rename = "date")]
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct BirthLocation {
    #[serde(rename = "location")]
    pub location: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "label")]
    pub label: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Corporation {
    #[serde(rename = "corporation")]
    pub corporation: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Description {
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "language")]
    pub language: Language,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "event")]
    pub event: i64,

    #[serde(rename = "mediums")]
    pub mediums: Option<Vec<Occupation>>,
}

#[derive(Serialize, Deserialize)]
pub struct Occupation {
    #[serde(rename = "subject")]
    pub subject: i64,
}

#[derive(Serialize, Deserialize)]
pub struct SourceLocation {
    #[serde(rename = "location")]
    pub location: i64,

    #[serde(rename = "label")]
    pub label: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Name {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "order")]
    pub order: i64,
}

#[derive(Serialize, Deserialize)]
pub struct SourcePerformance {
    #[serde(rename = "person")]
    pub person: i64,

    #[serde(rename = "works")]
    pub works: Vec<PerformanceWork>,
}

#[derive(Serialize, Deserialize)]
pub struct PerformanceWork {
    #[serde(rename = "work")]
    pub work: i64,

    #[serde(rename = "count")]
    pub count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "project")]
    pub project: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Serial {
    #[serde(rename = "series")]
    pub series: i64,

    #[serde(rename = "mediums")]
    pub mediums: Option<Vec<Occupation>>,
}

#[derive(Serialize, Deserialize)]
pub struct SourceElement {
    #[serde(rename = "source")]
    pub source: i64,
}

#[derive(Serialize, Deserialize)]
pub struct SourceWork {
    #[serde(rename = "work")]
    pub work: i64,

    #[serde(rename = "performances")]
    pub performances: Option<Vec<WorkPerformance>>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkPerformance {
    #[serde(rename = "event")]
    pub event: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BiographyUnion {
    AnythingArray(Vec<Option<serde_json::Value>>),

    BiographyClass(BiographyClass),
}

#[derive(Serialize, Deserialize)]
pub enum Ignored {
    #[serde(rename = "descriptions.description.keyword")]
    DescriptionsDescriptionKeyword,
}

#[derive(Serialize, Deserialize)]
pub enum Index {
    #[serde(rename = "person")]
    Person,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "_doc")]
    Doc,
}

#[derive(Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "de")]
    De,
}
