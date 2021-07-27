pub mod graph {

    use graph_items::node::Node;

    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: Vec<(String, String)>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: Vec::new(),
            }
        }

        pub fn with_nodes(mut self, nodes:&Vec<Node>) -> Self {
            self.nodes.extend(nodes);
            self
        }
    }

    pub mod graph_items {

        pub mod edge {

            pub struct Edge;

        }

        pub mod node {

            pub struct Node {
                name: String
            }

            impl Node {

                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string()
                    }
                }

            }

        }

    }

}
