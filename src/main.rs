use warp::{Filter};
use crate::structs::store::Store;
use crate::structs::sensors::Sensors;

mod structs;
mod controllers;


#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());
    let sensors = Sensors::new();
    let sensors_filter = warp::any().map(move || sensors.clone());


    let say_hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let get_sensor_position_route = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("sensors"))
        .and(sensors_filter.clone())
        .and(warp::path::param())
        .and(warp::path("position"))
        .and_then(controllers::sensors::get_sensor_position);

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

    let get_sensors = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("sensors"))
        .and(warp::path::end())
        .and(sensors_filter.clone())
        .and_then(controllers::sensors::get_all_sensors);

    let get_sensors_info = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("sensors"))
        .and(warp::path!("all"))
        .and(warp::path::end())
        .and(sensors_filter.clone())
        .and_then(controllers::sensors::get_all_sensors_full_info);

    let get_device_position_route = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("device"))
        .and(sensors_filter.clone())
        .and(store_filter.clone())
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(controllers::distance::get_device_pos);


    let routes = say_hello
        .or(add_data)
        .or(create_sensor)
        .or(get_sensors)
        .or(get_sensors_info)
        .or(get_sensor_position_route)
        .or(get_device_position_route);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
