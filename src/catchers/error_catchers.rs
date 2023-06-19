#[catch(400)]
pub fn not_found() -> String {
    format!("Not found")
}
