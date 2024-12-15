#![allow(unused)]
use email::{Body, Email, EmailAddress};
use file::File;
use microsoft::model::{Attachment, Page};
use microsoft::{MicrosoftAuth, MicrosoftClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = env::var("MICROSOFT_ACCESS_TOKEN").unwrap();
    let refresh_token = env::var("MICROSOFT_REFRESH_TOKEN").unwrap();
    let auth = MicrosoftAuth::oauth2(access_token, refresh_token, None);
    let client = MicrosoftClient::with_auth(auth);

    // let mut res = client
    //     .list_messages()
    //     .filter("contains(sender/emailAddress/address,'@unfi.com') and sentDateTime ge 2024-11-01T00:00:00Z")
    //     .await
    //     .unwrap();
    // println!("first");
    // while let Some(next_link) = res.next_link {
    //     println!("next_link: {:?}", next_link);
    //     res = client.list_messages().next(next_link).await.unwrap();
    //     println!("next");
    // }
    // let a = SendAttachment {
    //     name: "foo".to_string(),
    //     content_bytes: "baa".to_string(),
    //     content_type: "text/plain".to_string(),
    // };
    // let id = "AAkALgAAAAAAHYQDEapmEc2byACqAC-EWg0AFFbpB5l46EC2CzFk1V4-wgAAKGixKgAA";
    let email = Email {
        subject: "email subject gets ignored?".to_string(),
        body: Body::Html("<h1>An important email!</h1>".to_string()),
        from: EmailAddress {
            address: "deductions@everybodyeating.com".to_string(),
            // address: "kurt@promotedtest.onmicrosoft.com".to_string(),
            name: None,
        },
        to: vec![EmailAddress {
            address: "kurt@promotedtpm.com".to_string(),
            name: Some("Kurt Wolf".to_string()),
        }],
        cc: vec![EmailAddress {
            address: "kurtwolfbuilds@gmail.com".to_string(),
            name: Some("Kurt Wolf".to_string()),
        }],
        bcc: vec![],
        attachments: vec![File {
            name: "attached.txt".to_string(),
            content: "this is an attachment".to_string().into_bytes(),
        }],
        // reply_to_message_id: Some(id.to_string()),
        reply_to_message_id: None,
        thread_id: None,
    };
    let res = client.send_email(email).await?;
    let s = serde_json::to_string(&res).unwrap();
    println!("{}", s);

    // let res = client.list_attachments("AAMkADE0NWRlYTE2LTg2M2MtNGIwOC1hZTYwLWI3NjJiMGVjMjhiNQBGAAAAAACn8FeAOyVRSY2IRbkznDMBBwAUVukHmXjoQLYLMWTVXj-CAAAAAAEMAAAUVukHmXjoQLYLMWTVXj-CAAAfUHq5AAA=").await?;
    // for attachment in res.value {
    //     let bytes = attachment.bytes();
    //     fs::write(&attachment.name, bytes)?;
    // }

    Ok(())
}
