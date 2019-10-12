pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    type Attrs = HashMap<String, String>;

    pub trait Attributes {
        fn get_attrs<'a>(&'a mut self) -> &'a mut Attrs;
        fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self
        where
            Self: Sized,
        {
            for (k, v) in attrs.iter() {
                self.get_attrs().insert(k.to_string(), v.to_string());
            }
            self
        }
    }

    #[derive(Default, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: Attrs,
    }

    impl Attributes for Graph {
        fn get_attrs<'a>(&'a mut self) -> &'a mut Attrs {
            &mut self.attrs
        }
    }

    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.into();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.into();
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use super::super::Attributes;
            use super::super::Attrs;

            #[derive(Debug, Default, Clone, PartialEq)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                attrs: Attrs,
            }

            impl Attributes for Edge {
                fn get_attrs<'a>(&'a mut self) -> &'a mut Attrs {
                    &mut self.attrs
                }
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.into(),
                        to: to.into(),
                        ..Default::default()
                    }
                }
            }
        }

        pub mod node {
            use super::super::Attributes;
            use super::super::Attrs;

            #[derive(Debug, Default, Clone, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: Attrs,
            }

            impl Attributes for Node {
                fn get_attrs<'a>(&'a mut self) -> &'a mut Attrs {
                    &mut self.attrs
                }
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_owned(),
                        ..Default::default()
                    }
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|s| s.as_ref())
                }
            }
        }
    }
}
