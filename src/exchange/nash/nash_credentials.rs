/// This structure represents the Nash account credentials
#[derive(Clone)]
pub struct NashCredentials {
    pub secret: String,
    pub session: String,
}