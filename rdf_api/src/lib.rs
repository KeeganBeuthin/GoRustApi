use rdf::graph::Graph;
use rdf::uri::Uri;
use rdf::node::Node;
use rdf::triple::Triple;
use rdf::writer::n_triples_writer::NTriplesWriter;
use rdf::writer::rdf_writer::RdfWriter;
use rdf::reader::turtle_parser::TurtleParser;
use rdf::reader::rdf_parser::RdfParser;

pub struct RdfApi {
    graph: Graph,
}

fn node_to_string(node: &Node) -> String {
    match node {
        Node::UriNode { uri } => format!("<{}>", uri.to_string()),
        Node::LiteralNode { literal, data_type, language } => {
            let mut result = format!("\"{}\"", literal);
            if let Some(lang) = language {
                result.push_str(&format!("@{}", lang));
            } else if let Some(dt) = data_type {
                result.push_str(&format!("^^<{}>", dt.to_string()));
            }
            result
        },
        Node::BlankNode { id } => format!("_:{}", id),
    }
}

impl RdfApi {
    pub fn new() -> Self {
        RdfApi {
            graph: Graph::new(None),
        }
    }

    pub fn add_triple(&mut self, subject: &str, predicate: &str, object: &str) -> Result<(), String> {
        let s = self.create_uri_or_literal_node(subject)?;
        let p = self.create_uri_node(predicate)?;
        let o = self.create_uri_or_literal_node(object)?;
        let triple = Triple::new(&s, &p, &o);
        self.graph.add_triple(&triple);
        Ok(())
    }

    fn create_uri_node(&self, value: &str) -> Result<Node, String> {
        if value.starts_with('<') && value.ends_with('>') {
            Ok(Node::UriNode { uri: Uri::new(value[1..value.len()-1].to_string()) })
        } else {
            Ok(Node::UriNode { uri: Uri::new(value.to_string()) })
        }
    }

    fn create_node(&self, value: &str) -> Result<Node, String> {
        if value.starts_with('<') && value.ends_with('>') {
            Ok(Node::UriNode { uri: Uri::new(value[1..value.len()-1].to_string()) })
        } else if value.starts_with("_:") {
            Ok(Node::BlankNode { id: value[2..].to_string() })
        } else {
            Ok(Node::LiteralNode { 
                literal: value.to_string(), 
                data_type: None, 
                language: None 
            })
        }
    }

    fn create_uri_or_literal_node(&self, value: &str) -> Result<Node, String> {
        if value.starts_with("http://") || value.starts_with("https://") {
            self.create_uri_node(value)
        } else {
            self.create_node(value)
        }
    }

    pub fn load_from_turtle(&mut self, turtle_data: &str) -> Result<(), String> {
        let mut reader = TurtleParser::from_string(turtle_data.to_string());
        match reader.decode() {
            Ok(graph) => {
                self.graph = graph;
                Ok(())
            },
            Err(e) => Err(format!("Failed to parse Turtle data: {:?}", e)),
        }
    }

    pub fn serialize_to_ntriples(&self) -> Result<String, String> {
        let writer = NTriplesWriter::new();
        writer.write_to_string(&self.graph)
            .map_err(|e| format!("Failed to serialize graph: {:?}", e))
    }

    pub fn query(&self, subject: Option<&str>, predicate: Option<&str>, object: Option<&str>) -> Vec<(String, String, String)> {
        let mut results = Vec::new();
    
        for triple in self.graph.triples_iter() {
            let s = node_to_string(triple.subject());
            let p = node_to_string(triple.predicate());
            let o = node_to_string(triple.object());
    
            println!("DEBUG: Triple: {} {} {}", s, p, o);
    
            let s_match = subject.map_or(true, |subj| s.contains(subj));
            let p_match = predicate.map_or(true, |pred| p.contains(pred));
            let o_match = object.map_or(true, |obj| o.contains(obj));
    
            if s_match && p_match && o_match {
                results.push((s, p, o));
            }
        }
    
        results
    }

    pub fn debug_print_all_triples(&self) {
        println!("All triples in the graph:");
        for triple in self.graph.triples_iter() {
            println!("{} {} {}",
                node_to_string(triple.subject()),
                node_to_string(triple.predicate()),
                node_to_string(triple.object())
            );
        }
    }
}