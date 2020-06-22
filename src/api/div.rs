use actix_web::{web,HttpRequest,HttpResponse,Result};
use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
struct APIReturn
{
	error: String,
	result: f64
}

fn safety (v1: f64, v2: f64) -> APIReturn
{
	if v2 == 0.0
	{
		APIReturn {error: "Division By Zero".to_string(), result: 0.0}
	}
	else
	{
		APIReturn {error: "Sucess".to_string(), result: v1/v2}
	}	
}


async fn compute_div(req: HttpRequest) -> Result<HttpResponse>
{
	let v1: f64 = req.match_info().get("v1").unwrap().parse().unwrap();
	let v2: f64 = req.match_info().get("v2").unwrap().parse().unwrap();
	let result: APIReturn = safety(v1,v2);
	Ok(HttpResponse::Ok().json(result))
}


pub fn div(cfg: &mut web::ServiceConfig)
{
      cfg.service(web::resource("/{v1}/{v2}").route(web::get().to(compute_div)));
}