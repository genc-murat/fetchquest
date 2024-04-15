use anyhow::{Context, Result};
use clap::{ArgEnum, Parser};
use reqwest::{header, Client, Method};
use std::{
    fs::File,
    io::{self, Write},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'I', long)]
    head: bool,

    #[clap(short = 'i', long)]
    include_headers: bool,

    #[clap(short = 'A', long, default_value = "RustHttpClient/0.1.0")]
    user_agent: String,

    #[clap(short = 'X', long, arg_enum, default_value = "get")]
    request_type: RequestMethod,

    #[clap(short = 'L', long)]
    follow_redirects: bool,

    #[clap(short = 'b', long)]
    cookie: Option<String>,

    #[clap(short = 'v', long)]
    verbose: bool,

    #[clap(short = 's', long)]
    silent: bool,

    #[clap(short = 'o', long)]
    output: Option<String>,

    #[clap(short = 'H', long)]
    header: Vec<String>,

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
    let client = Client::builder()
        .user_agent(args.user_agent)
        .redirect(if args.follow_redirects {
            reqwest::redirect::Policy::limited(10)
        } else {
            reqwest::redirect::Policy::none()
        })
        .build()?;

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

    let res = request.send().await.context("Failed to send request")?;

    let mut output: Box<dyn Write> = if let Some(file_path) = args.output {
        Box::new(File::create(&file_path).context("Failed to create file")?)
    } else {
        Box::new(io::stdout())
    };

    if args.include_headers {
        writeln!(output, "{:?}", res.headers()).context("Failed to write headers")?;
    }

    if !args.head {
        let body = res.text().await.context("Failed to load response body")?;
        writeln!(output, "{}", body).context("Failed to write body")?;
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
