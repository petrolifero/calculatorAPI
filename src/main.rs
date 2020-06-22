mod api;

use actix_web::{HttpServer,App,web};

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(|| {
			App::new()
			.service(web::scope("/api").configure(api::api))
		       })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
