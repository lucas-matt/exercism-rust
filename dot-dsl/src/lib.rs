pub mod graph {

    use graph_items::node::Node;
    use graph_items::edge::Edge;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {

        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes:&Vec<Node>) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges:&Vec<Edge>) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn with_attrs(mut self, attrs:&[(&str, &str)]) -> Self {
            self.attrs.extend(attrs.iter()
                .map(|(x, y)| (x.to_string(), y.to_string())));
            self
        }

        pub fn get_node(self, name:&str) -> Option<Node> {
            let name = name.to_string();
            self.nodes.iter().find(|node| node.name == name).cloned()
        }

    }

    pub mod graph_items {

        pub mod edge {

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                source: String,
                target: String,
                attrs: Vec<(String, String)>
            }

            impl Edge {

                pub fn new(source:&str, target:&str) -> Self {
                    Edge {
                        source: source.to_string(),
                        target: target.to_string(),
                        attrs: Vec::new()
                    }
                }

                pub fn with_attrs(mut self, attrs:&[(&str, &str)]) -> Self {
                    self.attrs.extend(attrs.iter()
                        .map(|(x, y)| (x.to_string(), y.to_string())));
                    self
                }

            }

        }

        pub mod node {

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                attrs: Vec<(String, String)>
            }

            impl Node {

                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: vec!()
                    }
                }

                pub fn with_attrs(mut self, attrs:&[(&str, &str)]) -> Self {
                    self.attrs.extend(attrs.iter()
                        .map(|(x, y)| (x.to_string(), y.to_string())));
                    self
                }

                pub fn get_attr(&self, attr:&str) -> Option<&str> {
                    let attr = attr.to_string();
                    self.attrs.iter().find(|(k, _)| k.clone() == attr)
                        .map(|(_, v)| &v[..])
                }

            }

        }

    }

}
