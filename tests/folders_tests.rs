#[cfg(test)]
mod folders_tests {
    use std::env;

    use openapi::models::Items;
    // use serde::de::Unexpected::Str;
    use std::fs::File as StdFile;

    use box_rust_sdk::authorization::DeveloperTokenAuthorizaton;
    use box_rust_sdk::box_api_client::BoxApiClient;
    use box_rust_sdk::managers::folders;
    use box_rust_sdk::managers::folders::{CreateFolderRequest, FileUploadAttributes};
    // use box_rust_sdk::models::FileMiniAllOfFileVersion;
    use box_rust_sdk::models::ItemType::{File, Folder, WebLink};

    #[tokio::test]
    async fn folder_items_works() {
        let items = folders::items(&prepare_client(), &String::from("0")).await;

        assert!(items.entries.is_some(), "Missing items");
        let item_type = items.entries.unwrap().get(0).unwrap().item_type;
        assert!(
            [Folder, File, WebLink].contains(&item_type),
            "Unknown item type"
        )
    }

    #[tokio::test]
    async fn folder_create_delete_get_works() {
        let body = CreateFolderRequest::new(String::from("Test"), String::from("0"));

        let api = &prepare_client();
        let folder = folders::create(api, &body).await;
        let folder = folders::get(api, &folder.id).await;
        assert!(folder.is_some(), "Folder was not created");

        let folder_id = &folder.unwrap().id;
        folders::delete(api, folder_id, false).await;
        let folder = folders::get(api, folder_id).await;
        assert!(folder.is_none(), "Folder was not removed");
    }

    #[tokio::test]
    async fn upload_to_new_folder_works() {
        let body = CreateFolderRequest::new("Test Upload".to_string(), String::from("0"));

        let api = &prepare_client();
        let folder = folders::create(api, &body).await;

        let file = StdFile::open("resources/porg.jpeg").unwrap();
        let attrs = FileUploadAttributes::new(String::from("image.jpg"), &folder.id);

        let result = folders::upload_file(api, file, &attrs).await.unwrap();
        println!("File uploaded {:?}", &result);

        let folder_id = &folder.id;
        folders::delete(api, folder_id, true).await;
        let folder = folders::get(api, folder_id).await;
        assert!(folder.is_none(), "Folder was not removed");
    }

    #[tokio::test]
    async fn download_works() {
        let body = CreateFolderRequest::new("Test Upload".to_string(), String::from("0"));

        let api = &prepare_client();
        let folder = folders::create(api, &body).await;

        let file = StdFile::open("resources/porg.jpeg").unwrap();
        let attrs = FileUploadAttributes::new(String::from("image.jpg"), &folder.id);

        let result = folders::upload_file(api, file, &attrs).await.unwrap();
        println!("File uploaded {:?}", &result);
        //TODO: move this to upload_file
        let files: Items = serde_json::from_str(&result).unwrap();
        let vec = files.entries.unwrap();
        let file_id = vec.get(0).unwrap().clone().id;

        let mut file = StdFile::create("download_test.jpeg").unwrap();
        folders::download_file(api, &file_id, &mut file).await;

        let folder_id = &folder.id;
        folders::delete(api, folder_id, true).await;
        let folder = folders::get(api, folder_id).await;
        assert!(folder.is_none(), "Folder was not removed");
    }

    fn prepare_client() -> BoxApiClient {
        dotenv::dotenv().expect("Failed to read .env file");
        let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");
        let authorizaton = DeveloperTokenAuthorizaton::new(developer_token);
        BoxApiClient::new(authorizaton)
    }
}
