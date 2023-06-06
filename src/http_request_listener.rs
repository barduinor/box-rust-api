use url::Url;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UrlParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}

pub fn request_process(server: tiny_http::Server) -> Result<UrlParams, String> {
    let url_params = match server.recv() {
        Ok(rq) => {
            // println!(
            //     "received request:\nmethod: {:?}\nurl: {:?}\nremote_addr: {:?} ",
            //     rq.method(),
            //     rq.url(),
            //     rq.remote_addr(),
            //     // rq.headers()
            // );

            let base_url = Url::parse("http://127.0.0.1").expect("Error parsing base URL");

            let url = base_url
                .join(rq.url())
                .expect("Error parsing URL from request");

            let query_params: UrlParams = match serde_qs::from_str(url.query().unwrap()) {
                Ok(query_params) => query_params,
                Err(e) => UrlParams {
                    code: None,
                    state: None,
                    error: Some("Error parsing URL parameters".to_string()),
                    error_description: Some(e.to_string()),
                },
            };

            if query_params.error.is_some() {
                let response = tiny_http::Response::empty(403);
                rq.respond(response)
                    .expect("Error sending response local server");

                // Err(("Authorization error {}", query_params.error_description));
            } else {
                let response = tiny_http::Response::empty(200);
                rq.respond(response)
                    .expect("Error sending response local server");
            }

            query_params
        }
        Err(e) => UrlParams {
            code: None,
            state: None,
            error: Some("Error parsing URL parameters".to_string()),
            error_description: Some(e.to_string()),
        },
    };
    Ok(url_params)
}
