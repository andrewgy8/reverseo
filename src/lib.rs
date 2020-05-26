use osm4routing::models::Node;
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;

pub struct Tree {
    pub tree: KdTree<f64, Node, [f64; 2]>,
}

pub struct Result {
    nodes: Vec<(Node)>,
}


impl Tree {

    pub fn build(nodes: Vec<Node>) -> Tree {
        let dimensions = 2;
        let mut tree = KdTree::new(dimensions);
        for node in nodes {
            tree.add([node.coord.lat, node.coord.lon], node).unwrap();
        }
        Tree { tree }
    }
}

impl Result {
    pub fn node(self) -> Node {
        self.nodes[0]
    }
}