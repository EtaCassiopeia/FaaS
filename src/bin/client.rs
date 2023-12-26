use std::{env, fmt};

use reqwest::multipart;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

const UPLOAD_URL: &str = "http://127.0.0.1:8080/upload";
const INVOKE_URL: &str = "http://127.0.0.1:8080/invoke";

#[derive(Debug)]
enum CustomError {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> CustomError {
        CustomError::Io(err)
    }
}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError::Reqwest(err)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::Io(err) => write!(f, "IO error: {}", err),
            CustomError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
        }
    }
}

async fn upload_function(path: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let file = File::open(path).await?;

    let file_name = std::path::Path::new(path)
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or_else(|| anyhow::anyhow!("Failed to get file name"))?
        .to_owned();

    let stream = FramedRead::new(file, BytesCodec::new());
    let body = reqwest::Body::wrap_stream(stream);

    let part = multipart::Part::stream(body)
        .file_name(file_name)
        .mime_str("application/octet-stream")?;

    let form = multipart::Form::new().part("file", part);

    let response = client.post(UPLOAD_URL).multipart(form).send().await?;

    let result = response.text().await?;
    Ok(result)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: client <upload/invoke> <path/name> [arg]");
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "upload" => {
            let path = &args[2];
            upload_function(path).await.unwrap();
        }
        "invoke" => {
            let name = &args[2];
            let arg = args.get(3).cloned().unwrap_or_default();
            println!("Invoking {} with arg {}", name, arg);
            invoke_function(name, &arg)
                .await
                .unwrap_or_else(|e| println!("Error: {}", e));
        }
        _ => println!("Invalid action. Use 'upload' or 'invoke'."),
    }
}

async fn invoke_function(name: &str, arg: &str) -> Result<(), reqwest::Error> {
    let resp = reqwest::get(&format!("{}/{}/{}", INVOKE_URL, name, arg)).await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}
