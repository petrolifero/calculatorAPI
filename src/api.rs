mod add;
mod sub;
mod mult;
mod div;

use actix_web::web;


pub fn api(cfg: &mut web::ServiceConfig)
{
	cfg.service(
		web::scope("/add")
		.configure(add::add)
	);
	cfg.service(
		web::scope("/sub")
		.configure(sub::sub)
	);
	cfg.service(
		web::scope("/mult")
		.configure(mult::mult)
	);
	cfg.service(
		web::scope("/div")
		.configure(div::div)
	);
}