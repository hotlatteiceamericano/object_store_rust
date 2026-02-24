use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListParams {
    pub prefix: Option<String>,
    pub filename: Option<String>,
}
