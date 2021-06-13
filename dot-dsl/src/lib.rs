pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Default, Clone, PartialEq)]
            pub struct Edge {
                n1: String,
                n2: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(n1: &str, n2: &str) -> Self {
                    Edge {
                        n1: String::from(n1),
                        n2: String::from(n2),
                        ..Default::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Default, Clone, PartialEq)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: String::from(name),
                        ..Default::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }

                pub fn name(&self) -> &str {
                    &self.name
                }
            }
        }
    }

    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name() == name)
        }
    }
}
