use actix_web::{web,App,HttpServer,Responder, HttpResponse};
 
#[tokio::main]
async fn main() -> std::io::Result<()> {
        HttpServer::new(|| {
                App::new()
                    .route("/health_check", web::get().to(health_check))
                    
            })
            .bind("127.0.0.1:8000")?
            .run()
            .await
    
}
    
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[cfg(test)]
mod tets{
    use super::*;
    
    async fn health_check_succeed(){
        let response = health_check().await;
        
        assert!(response.status().is_success())
    }
}