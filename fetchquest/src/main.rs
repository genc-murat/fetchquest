use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{ArgEnum, Parser};
use futures::stream::TryStreamExt;
use reqwest::{
    header,
    multipart::{Form, Part},
    Body, Client, ClientBuilder, Method,
};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    head: bool,

    #[clap(short, long)]
    include_headers: bool,

    #[clap(short, long, default_value = "RustHttpClient/0.1.0")]
    user_agent: String,

    #[clap(short, long, arg_enum, default_value = "get")]
    request_type: RequestMethod,

    #[clap(short, long)]
    follow_redirects: bool,

    #[clap(short, long)]
    cookie: Option<String>,

    #[clap(short, long)]
    verbose: bool,

    #[clap(short, long)]
    silent: bool,

    #[clap(short, long)]
    output: Option<String>,

    #[clap(short, long)]
    header: Vec<String>,

    #[clap(short, long)]
    form_file: Option<PathBuf>,

    #[clap(short, long)]
    data: Option<String>,

    #[clap(short, long, help = "Disable SSL/TLS certificate validation")]
    disable_ssl_verification: bool,

    #[clap(short, long, help = "Bearer token for authentication (OAuth2 or JWT)")]
    bearer_token: Option<String>,

    url: String,
}

#[derive(ArgEnum, Clone, Debug)]
enum RequestMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut client_builder =
        Client::builder()
            .user_agent(args.user_agent)
            .redirect(if args.follow_redirects {
                reqwest::redirect::Policy::limited(10)
            } else {
                reqwest::redirect::Policy::none()
            });

    if args.disable_ssl_verification {
        client_builder = client_builder.danger_accept_invalid_certs(true);
    }

    let client = client_builder.build()?;

    let mut request = client.request(Method::from(args.request_type), &args.url);

    if let Some(cookie) = args.cookie {
        request = request.header(header::COOKIE, cookie);
    }

    for hdr in args.header {
        let parts: Vec<&str> = hdr.splitn(2, ':').collect();
        if parts.len() == 2 {
            request = request.header(parts[0].trim(), parts[1].trim());
        }
    }

    if let Some(token) = args.bearer_token {
        request = request.bearer_auth(token);
    }

    if let Some(form_path) = args.form_file {
        let file = File::open(&form_path)
            .await
            .context("Failed to open file for upload")?;
        let stream = FramedRead::new(file, BytesCodec::new()).map_ok(|bytes| bytes.freeze());
        let body = Body::wrap_stream(stream);
        let part = Part::stream(body)
            .file_name(form_path.file_name().unwrap().to_string_lossy().to_string())
            .mime_str("application/octet-stream")?;

        let form = Form::new().part("file", part);
        request = request.multipart(form);
    }

    if let Some(data) = args.data {
        request = request.body(data);
    }

    let res = request.send().await.context("Failed to send request")?;

    if args.silent {
        return Ok(());
    }

    if let Some(file_path) = args.output {
        let mut file = File::create(&file_path)
            .await
            .context("Failed to create file")?;
        if args.include_headers {
            file.write_all(format!("{:?}\n", res.headers()).as_bytes())
                .await
                .context("Failed to write headers")?;
        }
        if !args.head {
            let body = res.text().await.context("Failed to load response body")?;
            file.write_all(body.as_bytes())
                .await
                .context("Failed to write body")?;
        }
    } else {
        let mut stdout = tokio::io::stdout();
        if args.include_headers {
            stdout
                .write_all(format!("{:?}\n", res.headers()).as_bytes())
                .await
                .context("Failed to write headers")?;
        }
        if !args.head {
            let body = res.text().await.context("Failed to load response body")?;
            stdout
                .write_all(body.as_bytes())
                .await
                .context("Failed to write body")?;
        }
    }

    Ok(())
}

impl From<RequestMethod> for Method {
    fn from(method: RequestMethod) -> Self {
        match method {
            RequestMethod::Get => Method::GET,
            RequestMethod::Post => Method::POST,
            RequestMethod::Put => Method::PUT,
            RequestMethod::Delete => Method::DELETE,
        }
    }
}
