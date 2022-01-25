use warp::{Filter};
use crate::controllers::sensors::get_all_sensors;
use crate::structs::{Sensor, Sensors};

mod structs;
mod controllers;

type Store = structs::Store;


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
        .and(controllers::wifi_data::json_body())
        .and(store_filter.clone())
        .and_then(controllers::wifi_data::save_wifi_datum);

    let create_sensor = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("sensors"))
        .and(warp::path::end())
        .and(controllers::sensors::json_body())
        .and(sensors_filter.clone())
        .and_then(controllers::sensors::create_sensor);

        /*
    let get_sensors = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("sensors"))
        .and(warp::path::end())
        .and(sensor_json_body())
        .and_then(controllers::sensors::get_all_sensors);*/


    let routes = say_hello
        .or(add_data)
        .or(create_sensor);
        // .or(get_sensors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
