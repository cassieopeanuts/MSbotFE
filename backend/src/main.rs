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
    pub client: Mutex<Arc<Firestore>>,
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
    
    let discord_client_id = std::env::var("DISCORD_CLIENT_ID")
    .expect("DISCORD_CLIENT_ID not found in environment variables");

    let discord_client_secret = std::env::var("DISCORD_CLIENT_SECRET")
    .expect("DISCORD_CLIENT_SECRET not found in environment variables");

    let discord_redirect_url = std::env::var("DISCORD_REDIRECT_URL")
    .expect("DISCORD_REDIRECT_URL not found in environment variables");

    // Configure the OAuth2 client
    let client = BasicClient::new(
        ClientId::new(discord_client_id),
        Some(ClientSecret::new(discord_client_secret)),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://discord.com/api/oauth2/token".to_string()).unwrap()),
    )
    .set_redirect_url(RedirectUrl::new(discord_redirect_url).unwrap());
    
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
    
    let discord_client_id = std::env::var("DISCORD_CLIENT_ID")
        .expect("Discord client ID not found in environment variables");

    let redirect_uri = std::env::var("REDIRECT_URI")
        .expect("Redirect URI not found in environment variables");

    let discord_secret = std::env::var("DISCORD_SECRET")
        .expect("Discord secret not found in environment variables");

    let requested_scopes = std::env::var("REQUESTED_SCOPES")
        .expect("Requested scopes not found in environment variables");

        let private_key_str = std::env::var("FIREBASE_PRIVATE_KEY").unwrap();
    
        let private_key = private_key_str.replace("\\", "\n");
        
        let mut service_account_info: HashMap<&str, &str> = HashMap::new();
        service_account_info.insert("private_key", private_key.as_str());
        service_account_info.insert("type", "service_account");
        service_account_info.insert("project_id", std::env::var("FIREBASE_PROJECT_ID").unwrap().as_str());
        service_account_info.insert("private_key_id", std::env::var("FIREBASE_PRIVATE_KEY_ID").unwrap().as_str());
        service_account_info.insert("client_email", std::env::var("FIREBASE_CLIENT_EMAIL").unwrap().as_str());
        service_account_info.insert("client_id", std::env::var("FIREBASE_CLIENT_ID").unwrap().as_str());
        service_account_info.insert("auth_uri", "https://accounts.google.com/o/oauth2/auth");
        service_account_info.insert("token_uri", "https://oauth2.googleapis.com/token");
        service_account_info.insert("auth_provider_x509_cert_url", "https://www.googleapis.com/oauth2/v1/certs");
        service_account_info.insert("client_x509_cert_url", std::env::var("FIREBASE_CLIENT_CERT_URL").unwrap().as_str());
        
            let credentials = Credentials::from_service_account_info(&service_account_info)  
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
    .map(|(code, _): (String, String)| code)
    .and_then(
        |code: String| {
            let firestore_client = Arc::clone(&firestore_client);
            async move {
                match get_discord_id(code, firestore_client).await {
                    Ok(discord_id) => {
                        // Redirect the user to a success page or the home page
                        Ok(warp::reply::with_header(
                            warp::reply::html("<html><body>Login successful</body></html>"),
                            "Content-Type", 
                            "text/html"))
                    },
                    Err(_) => {
                        // Redirect the user to an error page or the home page with an error message
                        Ok(warp::reply::with_header(
                            warp::reply::html("<html><body>Login failed</body></html>"),
                            "Content-Type", 
                            "text/html"))
                    }
                }
            }
        }
    );
    // Start the Warp server
    warp::serve(callback_discord_route).run((Ipv4Addr::new(0, 0, 0, 0), 3030)).await;
    //wtf is that shit? "Setting the Content-Type header to text/html in the responses:"
    Ok(warp::reply::with_header(
        warp::reply::html("<html><body>Login successful</body></html>"),
        "Content-Type", 
        "text/html"));
}
