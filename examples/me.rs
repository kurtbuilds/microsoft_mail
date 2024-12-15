use std::env::var;

use microsoft::MicrosoftAuth;

#[tokio::main]
async fn main() {
    let auth = MicrosoftAuth::oauth2(var("BEARER").unwrap(), "".to_string(), None);
    let client = microsoft::MicrosoftClient::with_auth(auth);
    let me = client.me().await.unwrap();
    dbg!(me);
}
