use warp::hyper::StatusCode;
use crate::structs::sensors::Sensors;
use crate::structs::store::Store;

pub async fn get_device_pos(
    _sensors: Sensors,
    _store: Store,
    _device_id: String
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(StatusCode::NOT_IMPLEMENTED)
}
