use actix_web::{web,HttpRequest,Result};

async fn compute_sub(req: HttpRequest) -> Result<String>
{
	let v1: i64 = req.match_info().get("v1").unwrap().parse().unwrap();
	let v2: i64 = req.match_info().get("v2").unwrap().parse().unwrap();
	Ok(format!("{}", v1-v2))
}


pub fn sub(cfg: &mut web::ServiceConfig)
{
      cfg.service(web::resource("/{v1}/{v2}").route(web::get().to(compute_sub)));
}