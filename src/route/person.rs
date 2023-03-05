#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub country: String,
    pub age: i32,
}
