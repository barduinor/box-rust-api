// use cargo run --bin upload_experiment to run this file

use std::env;

use reqwest::{Body, Client, multipart};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().expect("Failed to read .env file");

    let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");

    let url = "https://upload.box.com/api/2.0/files/content";

    let client = Client::new();
    let file = File::open("resources/porg.jpeg").await?;

    // read file body stream
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_body = Body::wrap_stream(stream);

    //make form part of file
    let some_file = multipart::Part::stream(file_body)
        .file_name("image.jpeg")
        .mime_str("application/octet-stream")?;

    //create the multipart form
    let form = multipart::Form::new()
        .text("attributes", "{\n
          \"name\": \"image.jpeg\",\n
          \"parent\": {\n
            \"id\": \"0\"\n
          },\n
          \"content_created_at\": \"2023-06-05T10:53:43-08:00\",\n
          \"content_modified_at\": \"2023-06-05T10:53:43-08:00\"\n
        }")
        .part("file", some_file);

    //send request
    let response = client.post(url).header("Authorization", "Bearer ".to_owned() + &developer_token).multipart(form).send().await?;

    println!("{}", &response.status());
    Ok(())
}
