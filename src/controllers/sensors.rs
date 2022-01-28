use warp::{http, Reply};
use warp::{Filter};
use warp::http::StatusCode;
use warp::reply::with_status;
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

pub async fn get_sensor_position(
    sensors: Sensors,
    sensor_id: String
) -> Result<impl warp::Reply, warp::Rejection> {
    let sensors_map = sensors.sensors.read();
    return match sensors_map.get(&sensor_id) {
        Some(sensor) => Ok(warp::reply::json(&sensor.pos).into_response()),
        None => Ok(with_status("", StatusCode::NOT_FOUND).into_response())
    }
}

pub fn json_body() -> impl Filter<Extract = (Sensor,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
