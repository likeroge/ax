#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
    phone: String,
}
