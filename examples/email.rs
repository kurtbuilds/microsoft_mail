#![allow(unused)]
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use microsoft::model::{Attachment, Page};
use microsoft::{MicrosoftAuth, MicrosoftClient};
use std::{env, fs};

// what's left?
// insert into database
// insert into normalized "messages" table
//
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = env::var("MICROSOFT_ACCESS_TOKEN").unwrap();
    let refresh_token = env::var("MICROSOFT_REFRESH_TOKEN").unwrap();
    let auth = MicrosoftAuth::oauth2(access_token, refresh_token, None);
    let client = MicrosoftClient::with_auth(auth);

    // let dt = NaiveDate::from_ymd_opt(2024, 11, 1).unwrap();
    // let dt = dt.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    // let dt = dt.and_utc();
    // let filter = "contains(sender/emailAddress/address,'@unfi.com') and sentDateTime ge 2024-01-01T00:00:00Z";
    // let filter = "sentDateTime lt 2024-12-04T00:00:00Z and conversationId eq 'AAQkADE0NWRlYTE2LTg2M2MtNGIwOC1hZTYwLWI3NjJiMGVjMjhiNQAQAFvkdxg1_AJInbsOaHCs7Tw='";
    // let id = "AAkALgAAAAAAHYQDEapmEc2byACqAC-EWg0AeAYWNgSCRUuQYAILS36zMwAACznvvwAA";
    // let res = client.me().await?;
    // dbg!(res);
    let filter = "contains(subject, 'PRGX2351725')";
    let mut res = client
        .list_messages()
        .filter(filter)
        // .order_by("sentDateTime asc")
        .await
        .unwrap();
    'outer: loop {
        for message in res.value {
            println!("id: {}", message.id);
            println!("index: {}", message.conversation_index);
            println!("date: {}", message.sent_date_time);
            println!("subject: {}", message.subject);
            println!(
                "from: {}",
                message.from.map(|f| f.email_address.address).unwrap_or_default()
            );
            println!("to: {}", message.to_recipients.first().unwrap().email_address.address);
            // println!("body: {}", message.body.content);
            // println!("{}", serde_json::to_string(&message).unwrap());
            // if message.has_attachments {
            //     let attachments = client.list_attachments(&message.id).await?;
            //     println!("n attachments: {}", attachments.value.len());
            //     for attachment in attachments.value {
            //         let bytes = attachment.bytes();
            //         println!("attachment: {}", attachment.name);
            //         // fs::write(&attachment.name, bytes)?;
            //     }
            // }
            // break 'outer;
        }
        // break;
        let Some(next_link) = res.next_link else {
            break;
        };
        res = client.list_messages().next(next_link).await.unwrap();
    }

    Ok(())
}
