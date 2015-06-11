extern crate saturn;

use saturn::graph::structure;

#[test]
fn should_create_a_node() {
    let node = structure::Node { data: "A string".to_string(), adjacent: Vec::new() };
    assert!(node.data == "A string")
}

#[test]
fn should_create_an_edge() {
    let node = structure::Node { data: "A string".to_string(), adjacent: Vec::new() };
    let data = node.data.clone();
    let edge = structure::Edge { relationship: "knows".to_string(), target: node};
    assert!(edge.target.data == data)
}

#[test]
fn should_create_a_graph() {
    let node = structure::Node { data: "A string".to_string(), adjacent: Vec::new() };
    let edge = structure::Edge { relationship: "knows".to_string(), target: node.clone()};
    let nodes = vec![node];
    structure::Graph::new(&nodes);

}
