use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Match {
    pub id: String,
    pub user_id: String,
    pub matched_user_id: String,
    pub status: String,
}
