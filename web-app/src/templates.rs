use askama::Template;
use axum::response::{ Html, IntoResponse, Response };
use crate::models::Client;

#[derive(Template)]
#[template(path = "pages/login.html")]
pub struct LoginTemplate;

#[derive(Template)]
#[template(path = "pages/dashboard.html")]
pub struct DashboardTemplate {
    pub user: String,
    pub clients: Vec<Client>,
    pub page_title: String,
}

#[derive(Template)]
#[template(path = "pages/create_client.html")]
pub struct CreateClientTemplate {
    pub user: String,
    pub page_title: String,
}

#[derive(Template)]
#[template(path = "pages/edit_client.html")]
pub struct EditClientTemplate {
    pub user: String,
    pub page_title: String,
}


pub struct AskamaTemplate<T: Template>(pub T);

impl<T: Template> IntoResponse for AskamaTemplate<T> {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(rendered) => Html(rendered).into_response(),
            Err(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Template rendering error",
            ).into_response(),
        }
    }
}