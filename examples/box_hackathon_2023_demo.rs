// use cargo run --example box_hackathon_2023_demo to run this file

use box_rust_sdk::authorization::DeveloperTokenAuthorizaton;
use box_rust_sdk::box_api_client::BoxApiClient;
use box_rust_sdk::managers::files;
use box_rust_sdk::managers::folders;
use box_rust_sdk::managers::folders::CreateFolderRequest;
use box_rust_sdk::managers::users;

use box_rust_sdk::models::File;
use box_rust_sdk::models::FolderFull;
use std::env;

fn pause() {
    println!("Press enter to continue...\n");

    let _ = std::io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
}

async fn get_current_user(api: &BoxApiClient) {
    let user = users::me(api).await;

    println!("\nCurrent user:");
    println!(
        "id: {} \t name: {} \t login: {}",
        user.id.unwrap(),
        user.name.unwrap(),
        user.login.unwrap()
    );
    pause();
}

async fn list_folder_by_id(api: &BoxApiClient, folder_id: &str) {
    let items = folders::items(api, &String::from(folder_id)).await;

    println!("\nFolder content:");
    for item in items.entries.unwrap() {
        println!(
            "id: {} \t type: {:?} \t name: {}",
            item.id,
            item.item_type,
            item.name.unwrap()
        );
    }
    pause();
}

async fn create_folder(
    api: &BoxApiClient,
    folder_name: &str,
    parent_folder_id: &str,
) -> FolderFull {
    let body = CreateFolderRequest::new(folder_name.to_string(), parent_folder_id.to_string());
    folders::create(api, &body).await
}

async fn delete_folder(api: &BoxApiClient, folder_id: &String) {
    folders::delete(api, folder_id, false).await;
}

async fn upload_file(
    api: &BoxApiClient,
    folder_id: &str,
    file_path: &str,
    box_file_name: &str,
) -> File {
    let file = std::fs::File::open(file_path).unwrap();
    let attrs = folders::FileUploadAttributes::new(box_file_name.to_string(), folder_id);

    folders::upload_file(api, file, &attrs).await.unwrap()
    // println!("File uploaded {:?}", &result);
}

async fn delete_file(api: &BoxApiClient, file_id: &String) {
    files::delete(api, file_id).await;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");

    let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");

    let auth = DeveloperTokenAuthorizaton::new(developer_token);
    let api = BoxApiClient::new(auth);

    get_current_user(&api).await;
    list_folder_by_id(&api, "0").await;

    let folder_rusty_box = create_folder(&api, "Team Rusty Box", "0").await;
    println!(
        "Created folder: {}",
        folder_rusty_box.name.as_ref().unwrap()
    );

    let folder_box_hack = create_folder(&api, "Box Hackathon 2023", "0").await;
    println!("Created folder: {}", folder_box_hack.name.as_ref().unwrap());

    println!("\nRefresh browser to update folder list");
    pause();

    delete_folder(&api, &folder_rusty_box.id).await;
    println!("Deleted folder: {}", folder_rusty_box.name.unwrap());

    delete_folder(&api, &folder_box_hack.id).await;
    println!("Deleted folder: {}", folder_box_hack.name.unwrap());

    println!("\nRefresh browser to update folder list");
    pause();

    println!("Uploading file Rust.png to root folder (id: 0)");
    let file_rust = upload_file(&api, "0", "./resources/Rust.png", "Rusty.png").await;

    println!("Uploading file Box.png to root folder (id: 0)");
    let file_box = upload_file(&api, "0", "./resources/Box.png", "Box.png").await;

    println!("\nRefresh browser to update folder list");
    pause();

    delete_file(&api, &file_rust.id).await;
    println!("Deleted file: {}", file_rust.name.unwrap());

    delete_file(&api, &file_box.id).await;
    println!("Deleted file: {}", file_box.name.unwrap());

    // println!("\nRefresh browser to update folder list");
    // pause();
}
