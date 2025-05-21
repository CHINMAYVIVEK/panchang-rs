use actix_web::{HttpResponse, Responder, get, post, web};

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub status: String,
    pub status_code: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    pub timestamp: DateTime<Utc>,
    pub request_id: String,
}



#[get("/health")]
async fn health_chcek() -> impl Responder {

    let response: ApiResponse<()> = ApiResponse {
        status: "healthy".to_string(),
        status_code: 200,
        message: Some("Service is running".to_string()),
        data: None,
        timestamp: Utc::now(),
        request_id: "12345".to_string(),
    };
    
    HttpResponse::Ok().json(response)
}

#[post("/panchang")]
async fn panchang_handler(data: web::Json<crate::panchang::PanchangRequest>) -> impl Responder {
    match crate::panchang::calculate_panchang(data).await {
        Ok(panchang_data) => {
            let response: ApiResponse<crate::panchang::PanchangResponse> = ApiResponse {
                status: "success".to_string(),
                status_code: 200,
                message: Some("Panchang data fetched successfully".into()),
                data: Some(panchang_data),
                timestamp: Utc::now(),
                request_id: "12345".to_string(),
            };
            HttpResponse::Ok().json(response)
        },
        Err(error) => {
            let response: ApiResponse<crate::panchang::PanchangResponse> = ApiResponse {
                status: "error".to_string(),
                status_code: 400,
                message: Some(error.into()),
                data: None,
                timestamp: Utc::now(),
                request_id: "12345".to_string(),
            };
            HttpResponse::BadRequest().json(response)
        }
    }
}


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(health_chcek);
    cfg.service(panchang_handler);
}
