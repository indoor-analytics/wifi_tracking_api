use std::fmt::format;
use warp::hyper::StatusCode;
use warp::reply::with_status;
use crate::structs::sensors::Sensors;
use crate::structs::store::Store;

pub async fn get_device_pos(
    _sensors: Sensors,
    _store: Store,
    device_id: String
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(
        with_status(
            format!("No data was reported for device {}.", device_id),
            StatusCode::NOT_FOUND)
    )
}
