use actix_web::{web,HttpRequest,Result};

async fn compute_mult(req: HttpRequest) -> Result<String>
{
	let v1: u32 = req.match_info().get("v1").unwrap().parse().unwrap();
	let v2: u32 = req.match_info().get("v2").unwrap().parse().unwrap();
	Ok(format!("{}", v1*v2))
}


pub fn mult(cfg: &mut web::ServiceConfig)
{
      cfg.service(web::resource("/{v1}/{v2}").route(web::get().to(compute_mult)));
}