use axum::http::header;
use axum::response::{ IntoResponse, Redirect, Response };

use crate::models::Client;
use crate::templates::{
    AskamaTemplate, CreateClientTemplate, DashboardTemplate, EditClientTemplate, LoginTemplate
};

pub async fn index() -> Redirect {
    axum::response::Redirect::to("/login")
}

pub async fn login() -> impl IntoResponse {
    AskamaTemplate(LoginTemplate)
}

pub async fn dashboard() -> impl IntoResponse  {
    let clients = vec![
        Client {
            name: "client1".to_string(),
            description: "Laptop".to_string(),
            ip_address: "10.0.0.2".to_string(),
            allowed_ips: "10.0.0.2/32".to_string(),
            public_key: "AbCdEf...".to_string(),
            endpoint: "173.55.214.23".to_string(),
            last_seen: "2026-02-20 10:15:25".to_string(),
            bytes_sent: "100.2 KiB".to_string(),
            bytes_received: "50.9 KiB".to_string(),
            status: "Enabled".to_string(),
            creation_date: "2026-02-20 10:15:25".to_string(),
        }
    ];
    AskamaTemplate (
        DashboardTemplate {
            user: "user1".to_string(),
            clients,
            page_title: "WireGuard Dashboard".to_string(),
        }
    )
}

pub async fn create_client() -> impl IntoResponse  {
    AskamaTemplate (
        CreateClientTemplate {
            user: "user".to_string(),
            page_title: "WireGuard Create Client".to_string(),
        }
    )
}

pub async fn edit_client() -> impl IntoResponse  {
    AskamaTemplate (
        EditClientTemplate {
            user: "user".to_string(),
            page_title: "WireGuard Edit Client".to_string(),
        }
    )
}

pub async fn styles_css() -> Response {
    const STYLE: &str = include_str!("../public/styles.css");
    (
        [ (header::CONTENT_TYPE, "text/css") ],
        STYLE
    ).into_response()
}
