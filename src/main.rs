use futures::future::try_join_all;
use std::env;

#[macro_use]
extern crate rocket;

#[get("/test")]
async fn test() -> &'static str {
  let nodes = vec![
    "http://127.0.0.1:7470/",
    "http://127.0.0.1:7471/",
    "http://127.0.0.1:7472/",
  ];
  // let port = env::var("ROCKET_PORT").expect("Rocket port env var not found");
  // tested removing self to see if that helps at all.  it does not.
  // let filtered_nodes = nodes
  //   .into_iter()
  //   .filter(|&n| !n.contains(&port))
  //   .collect::<Vec<_>>(); // remove self
  let mut futures = Vec::new();
  for _ in 0..3 {
    for n in &nodes {
      futures.push(reqwest::get(*n));
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
