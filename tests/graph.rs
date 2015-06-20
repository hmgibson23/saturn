extern crate saturn;

use saturn::graph::structure;

#[test]
fn should_create_a_node() {
    let node = structure::Node::new("A string".to_string(), Vec::new());
    assert!(node.data == "A string")
}

#[test]
fn should_create_an_edge() {
    let node = structure::Node::new("A string".to_string(), Vec::new());
    let data = node.data.clone();
    let edge = structure::Edge::new("knows".to_string(), node);
    assert!(edge.target.data == data)
}

#[test]
fn should_create_a_graph() {
    let node = structure::Node::new("A string".to_string(), Vec::new());
    let edge = structure::Edge::new("knows".to_string(), node.clone());
    let dest = structure::Node::new("A destination".to_string(), vec![edge.clone()]);
    let nodes = vec![node.clone(), dest.clone()];
    let graph = structure::Graph::new(nodes);
    assert!(graph.graph.contains(&dest));
    assert!(graph.graph.contains(&node))
}

#[test]
fn should_add_nodes() {

}

#[test]
fn should_remove_nodes() {
}
