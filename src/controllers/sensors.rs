use warp::{http};
use crate::{Sensor, Sensors};
use warp::{Filter};

pub async fn create_sensor(
    sensor: Sensor,
    sensors: Sensors
) -> Result<impl warp::Reply, warp::Rejection> {
    let c = sensor.clone();
    sensors.sensors.write().insert(sensor.id, c);
    Ok(warp::reply::with_status(
        "Ok",
        http::StatusCode::OK
    ))
}

pub async fn get_all_sensors(
    sensors: Sensors
) -> Result<impl warp::Reply, warp::Rejection> {
    let sensorIds = vec![0, 1, 2]; //sensors.sensors.read().keys().clone();
    Ok(warp::reply::json(
        &sensorIds
    ))
}

pub fn json_body() -> impl Filter<Extract = (Sensor,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}