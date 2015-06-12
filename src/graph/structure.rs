extern crate uuid;

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

#[derive (Clone)]
pub struct Edge <T> {
    // this is the relationship as defined in the edge i.e Hugo knows Bob
    // might be worth restrictng this to just strings and numbers
    pub relationship: T,
    pub target: Node<T> // a target node needs to exist across any bound i.e a network bound
}


// initial graph representation is an adjacency list
pub struct Graph <T> {
    pub graph: Box<Vec<Node<T>>>
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
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self._id == other._id
    }
}
