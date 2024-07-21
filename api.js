const fetch = require('node-fetch');

const testApi = async (name, url) => {
    console.log(`Testing ${name} API:`);

    // Test GET /triples
    try {
        const getResponse = await fetch(`${url}/triples`);
        const triples = await getResponse.json();
        console.log('GET /triples response:', triples);
    } catch (error) {
        console.error('Error testing GET /triples:', error.message);
    }

    // Test POST /triples
    try {
        const newTriple = {
            subject: "http://example.org/book1",
            predicate: "http://purl.org/dc/elements/1.1/title",
            object: "The Go Programming Language"
        };
        const postResponse = await fetch(`${url}/triples`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(newTriple),
        });
        console.log('POST /triples status:', postResponse.status);

        // Fetch triples again to see if the new triple was added
        const getResponse = await fetch(`${url}/triples`);
        const triples = await getResponse.json();
        console.log('GET /triples after POST:', triples);
    } catch (error) {
        console.error('Error testing POST /triples:', error.message);
    }

    console.log('\n');
};

const main = async () => {
    // Test Rust/Warp API (assuming it's still using the old structure)
    await testApi('Rust/Warp', 'http://127.0.0.1:8000');

    // Test Go/Gin API with new RDF structure
    await testApi('Go/Gin', 'http://localhost:8080');
};

main().catch(error => console.error('Error:', error));