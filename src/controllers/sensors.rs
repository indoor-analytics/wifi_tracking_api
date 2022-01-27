use warp::{http};
use warp::{Filter};
use crate::structs::sensors::{Sensor, Sensors};


pub async fn create_sensor(
    sensor: Sensor,
    sensors: Sensors
) -> Result<impl warp::Reply, warp::Rejection> {
    let c = sensor.clone();
    let exists = sensors.has_sensor(&sensor.id);
    sensors.sensors.write().insert(sensor.id, c);
    Ok(warp::reply::with_status(
        "Ok",
        match exists {
            true => { http::StatusCode::OK }
            false => { http::StatusCode::CREATED }
        }
    ))
}

pub async fn get_all_sensors(
    sensors: Sensors
) -> Result<impl warp::Reply, warp::Rejection> {
    let sensors_ids: Vec<String> = sensors.sensors.read().keys().cloned().collect();
    Ok(warp::reply::json( &sensors_ids ))
}

pub async fn get_all_sensors_full_info(
    sensors: Sensors
) -> Result<impl warp::Reply, warp::Rejection> {
    let sensors_info: Vec<Sensor> = sensors.sensors.read().values().cloned().collect();
    Ok(warp::reply::json( &sensors_info ))
}

pub fn json_body() -> impl Filter<Extract = (Sensor,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
