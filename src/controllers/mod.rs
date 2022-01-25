use warp::http;
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
