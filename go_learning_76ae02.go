// This tutorial demonstrates how to integrate with external APIs in Go.
// We will focus on making a simple GET request to a public API,
// handling the response, and parsing JSON data.
// By the end of this tutorial, you will understand:
// 1. How to create an HTTP client.
// 2. How to send a GET request to an API endpoint.
// 3. How to read the response body.
// 4. How to unmarshal JSON data into Go structs.
// 5. Basic error handling for network requests and JSON parsing.
package main

import (
	"encoding/json" // Package for JSON encoding and decoding
	"fmt"           // Package for formatted I/O (like printing to console)
	"io/ioutil"     // Package for I/O utility functions, like reading from a reader
	"net/http"      // Package for HTTP client and server implementations
	"time"          // Package for time-related functions, used here for setting timeouts
)

// Define a struct to represent the structure of the JSON response we expect.
// This helps us to easily work with the API data in a type-safe way.
// The `json:"fieldName"` tags tell the json package how to map JSON fields
// to our Go struct fields.
type Post struct {
	UserID int    `json:"userId"`
	ID     int    `json:"id"`
	Title  string `json:"title"`
	Body   string `json:"body"`
}

// apiBaseURL is the base URL for the external API we'll be interacting with.
// Using a constant makes it easy to manage and change if the API endpoint changes.
const apiBaseURL = "https://jsonplaceholder.typicode.com"

// fetchDataFromAPI makes a GET request to a specified API endpoint and
// attempts to parse the JSON response into the provided `v` interface.
// `v` is expected to be a pointer to a struct that matches the JSON structure.
func fetchDataFromAPI(endpoint string, v interface{}) error {
	// 1. Create a custom HTTP client with a timeout.
	// This is crucial for production applications to prevent requests
	// from hanging indefinitely if the API is slow or unresponsive.
	client := &http.Client{
		Timeout: 10 * time.Second, // Set a timeout of 10 seconds for the request.
	}

	// 2. Construct the full URL by joining the base URL and the endpoint.
	url := apiBaseURL + endpoint

	// 3. Create a new GET request.
	// `http.NewRequest` creates a new HTTP request. The first argument is the HTTP method,
	// the second is the URL, and the third is the request body (nil for GET requests).
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		// If there's an error creating the request, return it immediately.
		return fmt.Errorf("failed to create request: %w", err)
	}

	// 4. Set common headers, like User-Agent.
	// Some APIs might require or recommend a User-Agent header.
	// It helps the API identify the client making the request.
	req.Header.Set("User-Agent", "Go-API-Client/1.0")
	req.Header.Set("Accept", "application/json") // Indicate we expect JSON back.

	// 5. Execute the request.
	// `client.Do(req)` sends the HTTP request and returns the response.
	resp, err := client.Do(req)
	if err != nil {
		// If there's an error executing the request (e.g., network issues), return it.
		return fmt.Errorf("failed to execute request: %w", err)
	}
	// It's essential to close the response body when done to release resources.
	// `defer` ensures this happens no matter how the function exits.
	defer resp.Body.Close()

	// 6. Check for a successful HTTP status code.
	// Status codes in the 2xx range indicate success.
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		// If the status code is not in the 2xx range, it's an error.
		// We try to read the body to get more details from the API, but it might be empty.
		bodyBytes, _ := ioutil.ReadAll(resp.Body) // Ignore error here, as we might not get useful body for non-2xx
		return fmt.Errorf("API request failed with status code %d: %s", resp.StatusCode, string(bodyBytes))
	}

	// 7. Read the response body.
	// `ioutil.ReadAll` reads all bytes from the response body.
	bodyBytes, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		// If reading the body fails, return the error.
		return fmt.Errorf("failed to read response body: %w", err)
	}

	// 8. Unmarshal (parse) the JSON data into the provided struct.
	// `json.Unmarshal` takes the byte slice of JSON data and a pointer
	// to the Go variable where the data should be stored.
	err = json.Unmarshal(bodyBytes, v)
	if err != nil {
		// If JSON parsing fails, return the error. This often happens if the
		// struct definition doesn't match the JSON structure.
		return fmt.Errorf("failed to unmarshal JSON: %w", err)
	}

	// If all steps were successful, return nil (no error).
	return nil
}

func main() {
	fmt.Println("Fetching a single post from the API...")

	// Declare a variable of our `Post` struct type to hold the API data.
	// We need to pass a pointer to this variable to `fetchDataFromAPI`
	// so that the function can modify it.
	var singlePost Post

	// Call our helper function to fetch data for a specific post (e.g., ID 1).
	// We pass a pointer to `singlePost` so `fetchDataFromAPI` can populate it.
	err := fetchDataFromAPI("/posts/1", &singlePost)
	if err != nil {
		// If an error occurred during fetching or parsing, print it and exit.
		fmt.Printf("Error: %v\n", err)
		return // Exit the main function
	}

	// If no error, we can now access the data from the `singlePost` variable.
	fmt.Println("\nSuccessfully fetched post:")
	fmt.Printf("User ID: %d\n", singlePost.UserID)
	fmt.Printf("Post ID: %d\n", singlePost.ID)
	fmt.Printf("Title: %s\n", singlePost.Title)
	fmt.Printf("Body: %s\n", singlePost.Body)

	// Example of fetching multiple posts (an array of Post structs)
	fmt.Println("\nFetching a list of posts from the API...")

	// Declare a slice of Post structs to hold the list of posts.
	var posts []Post

	// Fetch data for the '/posts' endpoint, which returns an array of posts.
	// We pass a pointer to the `posts` slice.
	err = fetchDataFromAPI("/posts", &posts)
	if err != nil {
		fmt.Printf("Error fetching posts: %v\n", err)
		return
	}

	// Print the number of posts fetched and details of the first few.
	fmt.Printf("\nSuccessfully fetched %d posts.\n", len(posts))
	if len(posts) > 0 {
		fmt.Println("Details of the first post in the list:")
		fmt.Printf("  Title: %s\n", posts[0].Title)
	}
	if len(posts) > 1 {
		fmt.Println("Details of the second post in the list:")
		fmt.Printf("  Title: %s\n", posts[1].Title)
	}
}