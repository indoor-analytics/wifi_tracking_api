use warp::hyper::StatusCode;
use warp::Reply;
use warp::reply::with_status;
use crate::structs::sensors::Sensors;
use crate::structs::store::Store;

pub async fn get_device_pos(
    sensors: Sensors,
    store: Store,
    device_id: String
) -> Result<impl warp::Reply, warp::Rejection> {
    let sensors_list = sensors.sensors.read();
    if sensors_list.len() == 0 {
        return Ok(
            with_status(
                "No sensors have been registered yet.",
                StatusCode::CONFLICT
            )
                .into_response()
        );
    }

    let device_data = store.get_device_data(device_id.clone());
    if device_data.len() == 0 {
        return Ok(
            with_status(
                format!("No data was reported for device {}.", device_id.clone()),
                StatusCode::NOT_FOUND
            )
                .into_response()
        );
    }
    Ok(StatusCode::NOT_IMPLEMENTED.into_response())
}
