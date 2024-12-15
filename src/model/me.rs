use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(default)]
    pub business_phones: Vec<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    pub id: String,
    #[serde(default)]
    pub job_title: Option<String>,
    // email address
    #[serde(default)]
    pub mail: Option<String>,
    #[serde(default)]
    pub mobile_phone: Option<String>,
    #[serde(default)]
    pub office_location: Option<String>,
    #[serde(default)]
    pub preferred_language: Option<String>,
    #[serde(default)]
    pub surname: Option<String>,
    /// email address
    #[serde(default)]
    pub user_principal_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_serialization() {
        let s = r#"{"@odata.context":"https://graph.microsoft.com/v1.0/$metadata#users/$entity","businessPhones":[],"displayName":"Deductions","id":"ccacbda2-975a-475e-b4c3-5d11d3e9694f","mail":"deductions@everybodyeating.com","userPrincipalName":"deductions@everybodyeating.com"}"#;
        serde_json::from_str::<User>(s).unwrap();
        let s = r#"{"@odata.context":"https://graph.microsoft.com/v1.0/$metadata#users/$entity","businessPhones":[],"displayName":"test test","givenName":null,"id":"d213efc3-27e9-4827-876a-3ef7c751558b","surname":"test","userPrincipalName":"test@promotedtest.onmicrosoft.com"}"#;
        serde_json::from_str::<User>(s).unwrap();
    }
}
