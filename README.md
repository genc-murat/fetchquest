# FetchQuest

FetchQuest is a powerful and flexible command-line HTTP client built in Rust. It leverages the robustness of `reqwest` and `tokio` libraries to handle HTTP requests asynchronously. FetchQuest is designed for developers who need a quick and reliable tool to send HTTP requests, download files, and interact with APIs from the terminal.

## Features

- Supports multiple HTTP methods: GET, POST, PUT, DELETE.
- Customizable headers, including dynamic user-agent strings.
- Support for sending data and uploading files using multipart/form-data.
- Follow or ignore redirects based on user preference.
- Optional SSL/TLS certificate validation.
- OAuth2 and JWT support for secure endpoints.
- Silent mode for minimal output and verbose mode for detailed debugging.
- Save response headers and body directly to a file.
- Configurable through command-line arguments for flexible integration into scripts and workflows.

## Installation

FetchQuest requires Rust's cargo tool to be installed on your system. You can build and install FetchQuest using the following commands:

```bash
git clone https://github.com/yourusername/FetchQuest.git
cd FetchQuest
cargo build --release
cargo install --path .
```

## Usage

To use FetchQuest, you can pass various command-line arguments to customize your HTTP request. Below is the basic syntax:

```bash
fetchquest [FLAGS] [OPTIONS] --url <URL>
```

### Command-Line Arguments

- `-h, --head` : Fetch headers only.
- `-i, --include-headers` : Include HTTP headers in the output.
- `-u, --user-agent <USER_AGENT>` : Customize the user-agent header (default is "RustHttpClient/0.1.0").
- `-t, --request-type <REQUEST_TYPE>` : Type of HTTP request: get, post, put, delete (default is "get").
- `-f, --follow-redirects` : Follow HTTP redirects.
- `-c, --cookie <COOKIE>` : Include a cookie in the request header.
- `-v, --verbose` : Enable verbose mode.
- `-s, --silent` : Enable silent mode, suppress output.
- `-o, --output <FILE>` : Output file to save the response.
- `--header <HEADER>` : Custom headers to include in the request.
- `--form-file <PATH>` : Path to a file to upload.
- `--data <DATA>` : Data to send in the request body.
- `--disable-ssl-verification` : Disable SSL/TLS certificate verification.
- `--bearer-token <TOKEN>` : Bearer token for OAuth2 or JWT authentication.

### Example

Here's an example command to send a GET request with custom headers:

```bash
fetchquest --url "http://example.com/api" --request-type get --header "Accept: application/json" --header "X-Custom-Header: value"
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

FetchQuest is released under the MIT License. See the LICENSE file for more details.
