const fetch = require('node-fetch');

const testApi = async (name, url) => {
    console.log(`Testing ${name} API:`);

    // Test GET /books
    try {
        const getResponse = await fetch(`${url}/books`);
        const books = await getResponse.json();
        console.log('GET /books response:', books);
    } catch (error) {
        console.error('Error testing GET /books:', error.message);
    }

    // Test POST /books
    try {
        const newBook = {
            id: 1,
            title: "Test Book",
            author: "Test Author"
        };
        const postResponse = await fetch(`${url}/books`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(newBook),
        });
        console.log('POST /books status:', postResponse.status);

        // Fetch books again to see if the new book was added
        const getResponse = await fetch(`${url}/books`);
        const books = await getResponse.json();
        console.log('GET /books after POST:', books);
    } catch (error) {
        console.error('Error testing POST /books:', error.message);
    }

    console.log('\n');
};

const main = async () => {
    // Test Rust/Warp API
    await testApi('Rust/Warp', 'http://127.0.0.1:8000');

    // Test Go/Gin API
    await testApi('Go/Gin', 'http://localhost:8080');
};

main().catch(error => console.error('Error:', error));