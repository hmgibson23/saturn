extern crate uuid;


// consider making the graph more generic as currently it only has one type
// so edges must have the same type as their target node which is annoying
// alternatively define a type that is generic across all types - as in
// data is just a stored bitset in the graph that is mapped over accordingly
// relationship is currently a string whcih can be parsed an an integer or whatever
// as needs be. this eliminates the problem of above

/**
 * The definitions are deliberately apart form the trait implementations
 */
type NodeId = uuid::Uuid;

#[derive (Clone)]
pub struct Node <T> {
    _id: NodeId, //id is the reference point on the graph - should be unique
    pub data: T,
    pub adjacent: Box<Vec<Edge<T>>>
}


// this needs another type variable as relationship will not necessarily have
// the same type as an edge

#[derive (Clone)]
pub struct Edge <T> {
    // this is the relationship as defined in the edge i.e Hugo knows Bob
    // might be worth restrictng this to just strings and numbers
    pub relationship: String,
    pub target: Node<T> // a target node needs to exist across any bound i.e a network bound
}


// initial graph representation is an adjacency list
pub struct Graph <T> {
    pub graph: Box<Vec<Node<T>>>
}


impl<T> Edge<T> {
    pub fn new(relationship: String, target: Node<T>) -> Edge<T> {
        Edge { relationship: relationship, target: target}
    }
}


impl<T> Graph<T> {
    pub fn new(input: Vec<Node<T>>) -> Graph<T> {
        Graph { graph: Box::new(input) }
    }


    pub fn insert_node(&mut self, node: Node<T>) -> () {
        self.graph.push(node)
    }

    /**
     * This is currently a O(n) operation which is not ideal
     * but that's the cost of using an adjacency list
     */
    pub fn remove_node(&mut self, node: Node<T>) -> bool {
        self.graph.iter()
            .position(|val| val._id == node._id)
            .map(|e| self.graph.remove(e)).is_some()
    }
}


impl<T> Node<T> {
    pub fn new(data: T, adjacent: Vec<Edge<T>>) -> Node<T> {
        Node { _id: uuid::Uuid::new_v4(), data: data, adjacent: Box::new(adjacent) }
    }

    pub fn add_edge(&mut self, edge: Edge<T>) -> () {
        self.adjacent.push(edge)
    }

    pub fn remove_edge(&mut self, edge: Edge<T>) -> bool {
        self.adjacent.iter()
            .position(|val| val.target._id == edge.target._id)
            .map(|e| self.adjacent.remove(e)).is_some()
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self._id == other._id
    }
}
