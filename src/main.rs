use dee::Protection;

#[macro_use] extern crate rocket;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
use rocket::serde::json::Json;

#[openapi(tag = "Users")]
#[get("/user/<uid>")]
fn get_user(uid: u32) -> Option<Json<Protection>> {
    Some(Json(Protection {
        uid: uid,
        username: "me".to_string(),
    }))
}

#[openapi(skip)]
#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
    .mount(
        "/",
        openapi_get_routes![
            get_user,
            hello,
        ],
    )
    .mount(
        "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
    )
    .launch()
    .await;

    match launch_result {
        Ok(_) => println!("Done."),
        Err(err) => eprintln!("{}", err),
    };
}
