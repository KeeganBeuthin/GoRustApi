use rdf_api::RdfApi;

fn main() -> Result<(), String> {
    let mut api = RdfApi::new();

    // Load initial data
    let example = r#"
        @prefix : <http://example.org/> .
        @prefix foaf: <http://xmlns.com/foaf/0.1/> .
        :alice foaf:name "Alice" ;
               foaf:mbox <mailto:alice@work.example> .
        :bob foaf:name "Bob" .
    "#;
    api.load_from_turtle(example)?;

    // Add a new triple
    api.add_triple("http://example.org/bob", "http://xmlns.com/foaf/0.1/knows", "http://example.org/alice")?;

    // Debug print all triples
    api.debug_print_all_triples();

    // Query the graph
    let results = api.query(Some("http://example.org/bob"), None, None);
    println!("\nBob's properties:");
    if results.is_empty() {
        println!("No properties found for Bob.");
    } else {
        for (s, p, o) in results {
            println!("{} {} {}", s, p, o);
        }
    }

    // Serialize the graph
    match api.serialize_to_ntriples() {
        Ok(result) => println!("\nThe resulting graph:\n{}", result),
        Err(e) => println!("Error serializing graph: {}", e),
    }

    Ok(())
}