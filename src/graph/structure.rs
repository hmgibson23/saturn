
/**
 * The definitions are deliberately apart form the trait implementations
 */
#[derive (Clone)]
pub struct Node <T> {
    pub data: T,
    pub adjacent: Vec<Edge<T>>

}

#[derive (Clone)]
pub struct Edge <T> {
    // this is the relationship as defined in the edge i.e Hugo knows Bob
    // might be worth restrictng this to just strings and numbers
    pub relationship: T,
    pub target: Node<T>
}


// initial graph representation is an adjacency list
pub struct Graph <'a, T: 'a> {
    pub graph: &'a Vec<Node<T>>
}

impl<'a, T> Graph<'a, T> {
    pub fn new(input: &'a Vec<Node<T>>) -> Graph<T> {
        Graph { graph: input }
    }
}
