use futures::future::try_join_all;

#[macro_use]
extern crate rocket;

#[get("/test")]
async fn test() -> &'static str {
  let nodes = [
    "http://127.0.0.1:7470/",
    "http://127.0.0.1:7471/",
    "http://127.0.0.1:7472/",
  ];
  let mut futures = Vec::new();
  for i in 0..3 {
    for n in nodes {
      futures.push(reqwest::get(n));
    }
  }
  try_join_all(futures).await.unwrap();
  "Hello, world!"
}
#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![index])
    .mount("/", routes![test])
}
