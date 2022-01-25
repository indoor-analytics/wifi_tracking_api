use warp::{http, Reply};
use crate::{Sensor, Sensors};

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
    Ok(warp::reply::json(
        sensors.sensors.read().keys()
    ).into_response());
}
