#![allow(non_snake_case)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "ca");

use crate::env_loader::check_env_variables;

mod catchers;
mod entities;
mod env_loader;
mod routes;
mod services;
mod traits;

#[launch]
fn rocket() -> _ {
    //HEALTHY CHECKS
    check_env_variables();

    rocket::build()
        .mount("/", routes![routes::endpoints::action])
        .register("/", catchers![catchers::error_catchers::not_found])
}