use warp::Filter;
use std::sync::Arc;
use async_trait::async_trait;
use oauth2::{
    basic::BasicClient,
    reqwest::async_http_client,
    AsyncCodeTokenRequest, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse,
};
use serde::{Deserialize, Serialize};
use reqwest::header::AUTHORIZATION;
use tokio::sync::Mutex;

struct FirestoreClient {
    pub client: Mutex<Firestore>,
}

#[derive(Serialize, Deserialize, Clone)]
struct UserData {
    discord_id: String,
    ethereum_address: String,
}

#[derive(Deserialize)]
struct DiscordUser {
    id: String,
}

async fn get_discord_id(code: String, firestore_client: Arc<FirestoreClient>) -> Result<String, warp::Rejection> {
    // Configure the OAuth2 client
    let client = BasicClient::new(
        ClientId::new("your_client_id".to_string()),
        Some(ClientSecret::new("your_client_secret".to_string())),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap()),
    )
    .set_redirect_url(RedirectUrl::new("your_redirect_url".to_string()).unwrap());

    // Exchange the authorization code for an access token
    let token_result = client.exchange_code(AuthorizationCode::new(code)).request(async_http_client).await;

    // Handle the token response
    match token_result {
        Ok(token_response) => {
            // Access token obtained successfully
            let access_token = token_response.access_token().secret();

            // Fetch Discord user details using the access token
            let discord_user_result = reqwest::Client::new()
                .get("https://discord.com/api/v10/users/@me")
                .header(AUTHORIZATION, format!("Bearer {}", access_token))
                .send()
                .await;

            // Handle the Discord user details response
            match discord_user_result {
                Ok(response) => {
                    if response.status().is_success() {
                        // User details fetched successfully
                        let discord_user: DiscordUser = response.json().await.unwrap();
                        let discord_id = discord_user.id;

                        // Perform any necessary actions with the Discord user ID, such as storing it in Firestore
                        let firestore = firestore_client.client.lock().await;
                        let user_data = UserData {
                            discord_id: discord_id.clone(),
                            ethereum_address: "".to_string(), // Add the Ethereum address here if needed
                        };
                        firestore
                            .collection("users")
                            .document(discord_id.clone())
                            .set(&user_data)
                            .await
                            .expect("Failed to store user data in Firestore");

                        // Return the Discord ID
                        Ok(discord_id)
                    } else {
                        // Handle error...
                        Err(warp::reject())
                    }
                },
                Err(_) => {
                    // Handle error...
                    Err(warp::reject())
                }
            }
        },
        Err(_) => {
            // Handle error...
            Err(warp::reject())
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let discord_client_id = std::env::var("REACT_APP_DISCORD_CLIENT_ID")
        .expect("Discord client ID not found in environment variables");

    let redirect_uri = std::env::var("REACT_APP_REDIRECT_URI")
        .expect("Redirect URI not found in environment variables");

    let discord_secret = std::env::var("REACT_APP_DISCORD_SECRET")
        .expect("Discord secret not found in environment variables");

    let requested_scopes = std::env::var("REACT_APP_REQUESTED_SCOPES")
        .expect("Requested scopes not found in environment variables");

    let firebase_api_key = std::env::var("REACT_APP_FIREBASE_API_KEY")
        .expect("Firebase API key not found in environment variables");

    let firebase_auth_domain = std::env::var("REACT_APP_FIREBASE_AUTH_DOMAIN")
        .expect("Firebase auth domain not found in environment variables");

    let firebase_project_id = std::env::var("REACT_APP_FIREBASE_PROJECT_ID")
        .expect("Firebase project ID not found in environment variables");

    let firebase_storage_bucket = std::env::var("REACT_APP_FIREBASE_STORAGE_BUCKET")
        .expect("Firebase storage bucket not found in environment variables");

    let firebase_messaging_sender_id = std::env::var("REACT_APP_FIREBASE_MESSAGING_SENDER_ID")
        .expect("Firebase messaging sender ID not found in environment variables");

    let firebase_app_id = std::env::var("REACT_APP_FIREBASE_APP_ID")
        .expect("Firebase app ID not found in environment variables");

    let firebase_database_url = std::env::var("REACT_APP_FIREBASE_DATABASE_URL")
        .expect("Firebase database URL not found in environment variables");

    let firestore_document_id = std::env::var("REACT_APP_FIRESTORE_DOCUMENT_ID")
        .expect("Firestore document ID not found in environment variables");

    let credentials = Credentials::from_file("path/to/service_account_key.json")
        .expect("Failed to load Firestore credentials");

    let firestore = Firestore::new(credentials).expect("Failed to initialize Firestore client");
    
    let firestore_client = FirestoreClient {
            client: Mutex::new(firestore),
        };
        

    let firestore_client = Arc::new(firestore_client);
    let firestore_client_filter = warp::any().map(move || Arc::clone(&firestore_client));


    let callback_discord_route = warp::path("callback")
        .and(warp::path("discord"))
        .and(warp::query::<(String, String)>())
        .and(warp::any().map(move || Arc::clone(&firestore_client)))
        .and_then(
            |(code, _state, firestore_client): (String, String, Arc<FirestoreClient>)| async move {
                match get_discord_id(code, firestore_client).await {
                    Ok(discord_id) => {
                        // Redirect the user to a success page or the home page
                        Ok(warp::reply::html("<html><body>Login successful</body></html>"))
                    },
                    Err(_) => {
                        // Redirect the user to an error page or the home page with an error message
                        Ok(warp::reply::html("<html><body>Login failed</body></html>"))
                    }
                }
            },
        );

    // Start the Warp server
    warp::serve(callback_discord_route).run(([127, 0, 0, 1], 3030)).await;
}
