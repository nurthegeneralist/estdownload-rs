// EstDownload v.1
// You can query by insert json in body request with postman or curl.
// body request = {"speed":20, size:"20"}
// speed = 20 mbps
// size = 20 GB of file
use axum::{Router, extract::Json, routing::post};
use serde::{Deserialize, Serialize};
// define output
#[derive(Debug, Serialize)]
struct Output {
    hour: i32,
    minute: i32,
    second: i32,
}
// define input
#[derive(Deserialize)]
struct Input {
    size: f32,
    speed: f32,
}
//calculate function - to calculate how long time download needed for size and speed
async fn calculate(size: f32, speed: f32) -> Output {
    let gbtomb = &size * 8.0 * 1024.0;
    let formula = (&gbtomb / &speed) as i32;
    let h = &formula / 3600;
    let m = (&formula - (&h * 3600)) / 60;
    let s = &formula - (&h * 3600) - (&m * 60);
    Output {
        hour: h,
        minute: m,
        second: s,
    }
}
async fn estcounter(Json(payload): Json<Input>) -> Json<Output> {
    //call calculate function
    let result = calculate(payload.size, payload.speed).await;
    Json(result)
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/calculate", post(estcounter)); //todo : implement input from url

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
