extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sophia;
extern crate sophia_turtle;

use rdf_vocabularies::{ns::{rdf, rdfs}};
use sophia::prefix::Prefix;
use sophia::serializer::TripleSerializer;
use sophia::iri::Iri;
use sophia::term::literal::convert::AsLiteral;
use sophia::term::literal::Literal;
use sophia_turtle::serializer::turtle::TurtleConfig;
use sophia::graph::MutableGraph;
use sophia::ns::Namespace;
use sophia::serializer::turtle::TurtleSerializer;
use sophia::graph::inmem::FastGraph;

mod person_schema;

use person_schema::Person;
use person_schema::BiographyUnion;
use progress_bar::*;

use std::string::String;


const BASE_URI: &str = "http://onlogies.slub-dresden.de/musiconn.performance/";


/**
 * converts a json person into a rdf graph
 */
fn convert_person_to_rdf(person: Person, outfile: &str) -> Result<(), Box<dyn std::error::Error>> {
    let entity_iri: &str = &*format!("{}{}", BASE_URI, "entity#");
    let class_iri: &str = &*format!("{}{}", BASE_URI, "class#");
    let props_iri: &str = &*format!("{}{}", BASE_URI, "props#");


    let rdf_p: &str = rdf::PREFIX;
    let rdfs_p: &str = rdfs::PREFIX;
    let mpc: Namespace<&str> = Namespace::new(class_iri).unwrap();
    let mpe: Namespace<&str> = Namespace::new(entity_iri).unwrap();
    let mpp: Namespace<&str> = Namespace::new(props_iri).unwrap();

    let lit = |txt: String| AsLiteral::as_literal(&txt);
    let mut graph = FastGraph::new();
    /*let authority_term = |id: i64| {
        let authority_iri = String::from( &*format!("authority-{}", id) );
        let authority = mpe.get(&authority_iri).unwrap();
        return Box::new(authority);
    };*/

    init_progress_bar(person.len());
    set_progress_bar_action("person->rdf", Color::Green, Style::Bold);
    person.iter().for_each(|person_element| {
        let p = &person_element.source;
        match p {
            person_schema::Source { uid, title, score, slug, locations, authorities, projects, .. } => {
                let person_iri = &*format!("{}", uid);
                let person = mpe.get(person_iri).unwrap();
                graph.insert(&person, &rdf::type_, &mpc.get("Person").unwrap()).unwrap();
                graph.insert(&person, &mpp.get("title").unwrap(), &lit(title.to_string())).unwrap();
                //graph.insert(&person, &mpp.get("slug").unwrap(), &lit(slug)).unwrap();
                for authorityVec in authorities {
                    for authority in authorityVec {
                        let authority_iri = &*format!("authority-{}", authority.authority);
                        let authority = mpe.get(authority_iri).unwrap();
                        graph.insert(&person, &mpp.get("authority").unwrap(), &authority).unwrap();
                    }
                }
            }
        }
        inc_progress_bar();
    });
    let mut writer = std::fs::File::create(outfile)?;
    let prefixes = [
        (Prefix::new_unchecked("rdf"), Iri::new_unchecked(rdf_p)),
        (Prefix::new_unchecked("rdf"), Iri::new_unchecked(rdfs_p)),
        (Prefix::new_unchecked("mpc"), Iri::new_unchecked(class_iri)),
        (Prefix::new_unchecked("mpe"), Iri::new_unchecked(entity_iri)),
        (Prefix::new_unchecked("mpp"), Iri::new_unchecked(props_iri)),
    ];
    let config = TurtleConfig::new()
        .with_pretty(true)
        .with_prefix_map(
            &prefixes[..]);
    let mut serializer = TurtleSerializer::new_with_config(&mut writer, config);

    serializer.serialize_graph(&graph)?;

    return Ok(());
}


fn main() {

    //parse arguments (filename, index, type, help)
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let outfile = &args[2];

    if filename == "" || !std::path::Path::new(filename).exists() {
        println!("Please provide a valid filename");
        return;
    }


    let json = std::fs::read_to_string(filename).unwrap();
    let model: Person = serde_json::from_str(&json).unwrap();

    convert_person_to_rdf(model, outfile).unwrap();
}
