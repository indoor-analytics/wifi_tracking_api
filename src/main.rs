use warp::{http, Filter};
use crate::structs::{Sensor, Sensors};

mod structs;
mod controllers;

type Datum = structs::Datum;
type Store = structs::Store;


async fn save_wifi_datum(
    datum: Datum,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    store.data.write().push_back(datum);
    Ok(warp::reply::with_status(
        "Received",
        http::StatusCode::OK,
    ))
}


fn json_body() -> impl Filter<Extract = (Datum,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn sensor_json_body() -> impl Filter<Extract = (Sensor,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());
    let sensors = Sensors::new();
    let sensors_filter = warp::any().map(move || sensors.clone());


    let say_hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let add_data = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("wifiData"))
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(save_wifi_datum);

    let create_sensor = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("sensors"))
        .and(warp::path::end())
        .and(sensor_json_body())
        .and(sensors_filter.clone())
        .and_then(controllers::sensors::create_sensor);

    let routes = say_hello
        .or(add_data)
        .or(create_sensor);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
