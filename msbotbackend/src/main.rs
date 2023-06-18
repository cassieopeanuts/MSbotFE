use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;
use warp::Filter;
use dotenv::dotenv;
use reqwest::header::AUTHORIZATION;
use firestore::{Document, Firestore};
use web3::types::{Address, H160, H256, U256};
use web3::contract::Contract;
use web3::futures::Future;
use web3::transports::Http;


// Struct to store user data in Firestore
#[derive(Debug, Serialize, Deserialize)]
struct UserData {
    discord_id: String,
    ethereum_address: String,
}

// State to store the Firestore client
struct FirestoreClient {
    client: Firestore,
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

    let firestore = Firestore::new(credentials)
        .expect("Failed to initialize Firestore client");

    let firestore_client = FirestoreClient {
        client: firestore,
    };

    let firestore_client = Arc::new(firestore_client);
    let firestore_client_filter = warp::any().map(move || Arc::clone(&firestore_client));

    // Create the Discord OAuth2 client
    let client = BasicClient::new(
        ClientId::new(discord_client_id),
        None,
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string())
            .expect("Invalid authorization endpoint URL"),
        None,
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_uri)
            .expect("Invalid redirect URL"),
    )
    .add_scope(Scope::new("identify".to_string()))
    .add_scope(Scope::new("email".to_string()));

    // Generate the authorization URL with CSRF protection
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .url();

    let login_discord_route = warp::path("login").and(warp::path("discord")).map(move || {
        warp::redirect(warp::http::Uri::from_str(&authorize_url.to_string())
            .expect("Failed to create Discord authorization URL"))
    });

    let callback_discord_route = warp::path("callback")
    .and(warp::path("discord"))
    .and(warp::query::<(String, String)>())
    .and(warp::any().map(move || Arc::clone(&firestore_client)))
    .and_then(
        |(code, _state, firestore_client): (String, String, Arc<FirestoreClient>)| async move {
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
                                    .document(discord_id)
                                    .set(&user_data)
                                    .await
                                    .expect("Failed to store user data in Firestore");

                                // Redirect the user to a success page or the home page
                                Ok(warp::reply::html("<html><body>Login successful</body></html>"))
                            } else {
                                eprintln!("Failed to fetch Discord user details: {:?}", response);
                                // Redirect the user to an error page or the home page with an error message
                                Ok(warp::reply::html("<html><body>Login failed</body></html>"))
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to fetch Discord user details: {:?}", err);
                            // Redirect the user to an error page or the home page with an error message
                            Ok(warp::reply::html("<html><body>Login failed</body></html>"))
                        }
                    }
                }
                Err(err) => {
                    // Handle the error case
                    eprintln!("Failed to exchange authorization code: {:?}", err);
                    // Redirect the user to an error page or the home page with an error message
                    Ok(warp::reply::html("<html><body>Login failed</body></html>"))
                }
            }
        },
    );

    #[derive(Debug, Serialize, Deserialize)]
    struct MetamaskConnectResponse {
        id: String,
        jsonrpc: String,
        result: String,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    struct MetamaskAccount {
        address: String,
    }

    let connect_metamask_route = warp::path("connect").and(warp::path("metamask")).map(|| {
        // Implementation for Metamask connection route
        // ...
        warp::redirect(warp::http::Uri::from_static("https://your-metamask-connection-url"))
    });
    
    // Define a data structure to receive the Ethereum address from Metamask
    #[derive(Debug, Deserialize)]
    struct MetamaskCallbackQuery {
        address: String,
    }
    
    let callback_metamask_route = warp::path("callback")
        .and(warp::path("metamask"))
        .and(warp::query::<MetamaskCallbackQuery>())
        .and(warp::any().map(move || Arc::clone(&firestore_client)))
        .and_then(
            |query: MetamaskCallbackQuery, firestore_client: Arc<FirestoreClient>| async move {
                // Implementation for Metamask callback route
                // ...
    
                // Fetch the Ethereum address from the Metamask callback
                let eth_address = query.address;
    
                // Get the Discord user ID from the user's session or any other means
                // For the sake of this example, let's say you have a function `get_discord_id_from_session()`
                let discord_id = get_discord_id_from_session().await?;
    
                // Perform any necessary actions with the Ethereum address, such as linking it to the Discord user ID in Firestore
                let mut firestore = firestore_client.client.lock().await;
                let user_data = UserData {
                    discord_id: discord_id.clone(),
                    ethereum_address: eth_address.clone(),
                };
                firestore
                    .collection("users")
                    .document(&discord_id)
                    .set(&user_data)
                    .await
                    .map_err(|e| {
                        warp::reject::custom(e) // Reject with the Firestore error
                    })?;
    
                // Redirect the user to a success page or the home page
                Ok(Response::builder()
                    .header("content-type", "text/html")
                    .body("<html><body>Connected to Metamask successfully</body></html>".to_string()))
            },
        );
    
        let store_user_data_route = warp::path("store_user_data")
        .and(warp::post())
        .and(warp::body::json())
        .and(firestore_client_filter.clone())
        .and_then(
            |user_data: UserData, firestore_client: Arc<FirestoreClient>| async move {
                // Lock the Firestore client for this async block
                let mut firestore = firestore_client.client.lock().await;
                
                // Store the user data in Firestore
                let result = firestore
                    .collection("users")
                    .document(user_data.discord_id.clone())
                    .set(&user_data)
                    .await;
    
                match result {
                    Ok(_) => {
                        // If the data was stored successfully, return a 200 status
                        Ok(warp::reply::with_status(
                            "User data stored successfully",
                            warp::http::StatusCode::OK
                        ))
                    }
                    Err(e) => {
                        // If there was an error, return a 500 status
                        eprintln!("Failed to store user data in Firestore: {:?}", e);
                        Ok(warp::reply::with_status(
                            "Internal server error",
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR
                        ))
                    }
                }
            },
        );    
        
//Web3 logic here
        let contract_address = "your contract address here";
        let contract_abi = include_str!("your contract ABI here.json");

        let (_eloop, transport) = web3::transports::Http::new("https://rpc.moonriver.moonbeam.network").unwrap();
        let web3 = web3::Web3::new(transport);
    
        let contract = Contract::from_json(
            web3.eth(),
            contract_address.parse::<H160>().unwrap(),
            contract_abi.as_bytes(),
        ).unwrap();
    
        let result: U256 = contract.query("balanceOf", (my_address,), None, Default::default(), None).wait().unwrap();
        println!("balanceOf: {}", result);
    
        let result: U256 = contract.query("totalDeposits", (), None, Default::default(), None).wait().unwrap();
        println!("totalDeposits: {}", result);
    
        let result: U256 = contract.query("totalWithdrawals", (), None, Default::default(), None).wait().unwrap();
        println!("totalWithdrawals: {}", result);
    
        let result: U256 = contract.query("totalFees", (), None, Default::default(), None).wait().unwrap();
        println!("totalFees: {}", result);


    let routes = login_discord_route
        .or(callback_discord_route)
        .or(connect_metamask_route)
        .or(callback_metamask_route)
        .or(store_user_data_route);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
