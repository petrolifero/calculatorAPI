use actix_web::{Responder,HttpResponse};

pub async fn eval() -> impl Responder {
      HttpResponse::Ok().body("Eval")
}