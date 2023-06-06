#[cfg(test)]
mod folders_tests {
    use std::env;
    use box_rust_sdk::authorization::DeveloperTokenAuthorizaton;
    use box_rust_sdk::box_api_client::BoxApiClient;
    use box_rust_sdk::managers::folders;
    use box_rust_sdk::models::ItemType::{File, Folder, WebLink};

    #[tokio::test]
    async fn filder_items_works() {
        let items = folders::items(&prepare_client(), &String::from("0")).await;

        assert_eq!(items.entries.is_some(), true, "Missing items");
        let item_type = items.entries.unwrap().get(0).unwrap().item_type;
        assert_eq!([Folder, File, WebLink].contains(&item_type), true, "Unknown item type")
    }

    fn prepare_client() -> BoxApiClient {
        dotenv::dotenv().expect("Failed to read .env file");
        let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");
        let authorizaton = DeveloperTokenAuthorizaton::new(developer_token);
        BoxApiClient::new(authorizaton)
    }
}
