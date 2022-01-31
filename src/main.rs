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


    // GET /sensors
    let get_sensors = warp::get()
        .and(warp::path::end())
        .and(sensors_filter.clone())
        .and_then(controllers::sensors::get_all_sensors);
    // POST /sensors
    let create_sensor = warp::post()
        .and(warp::path::end())
        .and(controllers::sensors::json_body())
        .and(sensors_filter.clone())
        .and_then(controllers::sensors::create_sensor);
    // GET /sensors/all
    let get_sensors_info = warp::get()
        .and(warp::path!("all"))
        .and(warp::path::end())
        .and(sensors_filter.clone())
        .and_then(controllers::sensors::get_all_sensors_full_info);
    // GET /sensors/:sensor_id/position
    let get_sensor_position_route = warp::get()
        .and(sensors_filter.clone())
        .and(warp::path::param())
        .and(warp::path("position"))
        .and(warp::path::end())
        .and_then(controllers::sensors::get_sensor_position);

    let sensors_routes = warp::path("sensors").and(
        get_sensors.or(
            create_sensor
        ).or(
            get_sensors_info
        ).or(
            get_sensor_position_route
        )
    );

    // GET /device
    let get_device_position_route = warp::get()
        .and(warp::path("device"))
        .and(sensors_filter.clone())
        .and(store_filter.clone())
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(controllers::distance::get_device_pos);
    // POST /wifiData
    let add_data = warp::post()
        .and(warp::path("wifiData"))
        .and(warp::path::end())
        .and(controllers::wifi_data::json_body())
        .and(store_filter.clone())
        .and_then(controllers::wifi_data::save_wifi_datum);

    let routes = warp::path("v1").and(
        sensors_routes.or(
            add_data.or(
                get_device_position_route
            )
        )
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
