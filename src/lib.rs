use actix_web::{web,App,HttpServer,Responder, HttpResponse};  
    
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
pub async fn  run()-> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .route("/health_check", web::get().to(health_check))
    })
            .bind("127.0.0.1:8000")?
            .run()
            .await
}

#[cfg(test)]
mod tets{
    use super::*;
    
    async fn health_check_succeed(){
        let response = health_check().await;
        // This required changing the return type of 'health_check'
        // from 'impl Responder' to 'HttpResponse' to compile
        // you also need to import it with 'use axtix-web::HttpResponse'!
        
        assert!(response.status().is_success())
    }
    pub fn is_even(x:u64)-> bool {
        x % 2 == 0
    }
}