use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Debugger {
    pub description: String,
    pub devtools_frontend_url: String,
    pub devtools_frontend_url_compat: String,
    pub favicon_url: String,
    pub id: String,
    pub title: String,
    pub r#type: String,
    pub url: String,
    pub web_socket_debugger_url: String,
}

pub fn get_debuggers(
    host: &'static str,
    port: &'static str,
) -> Result<Vec<Debugger>, Box<dyn std::error::Error>> {
    let url = format!("http://{}:{}/json", host, port);
    let response = reqwest::blocking::get(url.as_str())?;
    let parsed_response: Vec<Debugger> = serde_json::from_str(&response.text()?)?;
    Ok(parsed_response)
}
