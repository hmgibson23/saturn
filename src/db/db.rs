extern crate rustc_serialize;

use std::collections::HashMap;
use graph::structure;


// the DB defaults to usin JSON to store data about nodes
// this might all change though - who knows
pub type GraphType = rustc_serialize::json::Json;

/*
 * Responsible for managing the database (s)
 * The Graph is Generic, the DB is not.
 */
pub struct DB {
    pub existing: HashMap<String, structure::Graph<rustc_serialize::json::Json>> //a list of the dbs in memory
}



impl DB {
    pub fn create(&mut self, name: &String) -> Result<String, String> {
        if self.existing.contains_key(name) {
            Err("Graph exists".to_string())
        } else {
            self.existing.insert(name.clone(), structure::Graph::new(Vec::new()));
            Ok("New graph created".to_string())
        }
    }

    pub fn show(&self) -> Result<String, String> {
        let mut v = Vec::new();

        for key in self.existing.keys() {
            v.push(key.clone())
        }

        let joined = v.connect("\n");
        Ok(joined)
    }

    pub fn new() -> DB {
        let map: HashMap<String, structure::Graph<GraphType>>
            = HashMap::new();
        DB { existing: map }
    }


    //TODO:  Implement!
    pub fn insert_node(&mut self, graph: String, node: structure::Node<GraphType>) ->
        Result<String, String> {
            //let mut graph =
            Ok("Inserted.".to_string())
    }

}
