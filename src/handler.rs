use axum::{extract::Path, response::Html};
use crate::storage::get_mock_profile;

pub async fn profile_handler(Path(id): Path<String>) -> Html<String> {
    let profile = get_mock_profile(&id);
    
    // Renders the profile using the premium SOLUX theme
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <link rel="stylesheet" href="/static/style.css">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>SOLUX | {name}</title>
        </head>
        <body>
            <div class="card">
                <div class="brand">SOLUX</div>
                <h1>{name}</h1>
                <p class="role">{role}</p>
                <p class="company">{company}</p>
                <div class="links">
                    <a href="mailto:{email}">Email Me</a>
                    <a href="https://{website}">Website</a>
                </div>
                <button class="save-btn">SAVE CONTACT</button>
            </div>
        </body>
        </html>
        "#,
        name = profile.name,
        role = profile.role,
        company = profile.company,
        email = profile.email,
        website = profile.website
    ))
}
