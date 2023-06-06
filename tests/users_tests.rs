use box_rust_sdk::managers;

#[cfg(test)]
mod users_tests {
    use std::env;
    use box_rust_sdk::authorization::DeveloperTokenAuthorizaton;
    use box_rust_sdk::box_api_client::BoxApiClient;
    use box_rust_sdk::managers;

    #[tokio::test]
    async fn me_works() {
        dotenv::dotenv().expect("Failed to read .env file");
        let developer_token = env::var("DEVELOPER_TOKEN").expect("DEVELOPER_TOKEN not set");
        let authorizaton = DeveloperTokenAuthorizaton::new(developer_token);
        let client = BoxApiClient::new(authorizaton);

        let me = managers::users::me(&client).await;

        assert_eq!(me.login.is_some(), true, "Missing user login");
    }
}
