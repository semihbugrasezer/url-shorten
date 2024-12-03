# 🔗 URL Shortener API

A simple URL shortener service built with **Rust**, **Actix-web**, and **Redis**. This API allows you to shorten long URLs and redirect users to the original URLs using short, user-friendly links.

## Features

- **Shorten URLs**: Convert long URLs into short, user-friendly URLs.
- **Redirect to Original URL**: When a short URL is accessed, users are redirected to the original URL.
- **Redis Backend**: URL mappings are stored in Redis for fast access.
- **Random Short URL Generation**: Short URLs are created using a 6-character random string.
- **Rate Limiting**: Protect the API from abuse with basic rate limiting.

## Technologies Used

- **Rust** (v1.68.0 or later)
- **Actix-web**: A powerful web framework for Rust.
- **Redis**: An in-memory key-value store used for fast URL storage and retrieval.
- **Serde**: For serializing and deserializing JSON data.
- **Rand**: A crate used for generating random short URL strings.

## Getting Started

### Prerequisites

- Install **Rust** (version 1.68.0 or later).
- Install **Redis** and ensure it is running locally on port `6379`.
- Ensure your project is set up with a **Redis connection**.

### Installation

1. **Clone the repository**:

    ```bash
    git clone https://github.com/yourusername/url-shortener.git
    cd url-shortener
    ```

2. **Install dependencies**:

    ```bash
    cargo build
    ```

3. **Start Redis Server**:
    Make sure Redis is installed and running locally:

    ```bash
    redis-server
    ```

4. **Run the application**:

    ```bash
    cargo run
    ```

    The server will be running on `http://localhost:1025`.

## API Endpoints

### `POST /api/shorten`
- **Description**: Shortens a long URL.
- **Request Body**:
    ```json
    {
        "original_url": "https://www.example.com"
    }
    ```
- **Response**:
    ```json
    {
        "short_url": "http://localhost:1025/api/qzpxlt"
    }
    ```

### `GET /api/{short_url}`

- **Description**: Redirects the user to the original URL associated with the short URL.
- **Path Parameter**:
    - `short_url`: The short URL code (e.g., `qzpxlt`).
- **Response**: A **302 Redirect** to the original URL.
    - Example: Redirects to `https://www.example.com`.

## Code Structure

1. **Redis Client Initialization** (`database/mod.rs`):  
   The Redis client is initialized and wrapped in a `Mutex` and `Arc` for safe concurrent access.

2. **URL Models** (`models/url.rs`):  
   Defines the request and response models for handling URLs. The `UrlRequest` model takes the original URL, while the `UrlResponse` model returns the shortened URL.

3. **Shorten URL Route** (`routes/shorten.rs`):  
   Defines the logic for shortening a URL. When a POST request is made to `/shorten`, it generates a short URL, stores it in Redis, and returns the shortened URL.

4. **Redirect URL Route** (`routes/redirect.rs`):  
   Handles the redirection logic. When a GET request is made to a short URL, the system looks up the corresponding original URL in Redis and redirects the user.

5. **Utility for URL Generation** (`utils.rs`):  
   Generates a 6-character random short URL string.

6. **Main Server Setup** (`main.rs`):  
   Initializes and runs the Actix-web server, binds the routes, and connects to Redis.
   
## License

This project is open-source and available under the [MIT License](LICENSE).
