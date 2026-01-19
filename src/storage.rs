use crate::models::SoluxProfile;

pub fn get_mock_profile(user_id: &str) -> SoluxProfile {
    // Simulated database lookup
    SoluxProfile {
        id: user_id.to_string(),
        name: "Alex Sterling".to_string(),
        role: "Chief Strategy Officer".to_string(),
        company: "SOLUX Global".to_string(),
        email: "alex@solux.id".to_string(),
        website: "www.solux.id".to_string(),
    }
}
