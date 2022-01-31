use warp::hyper::StatusCode;
use warp::Reply;
use warp::reply::with_status;
use crate::structs::sensors::Sensors;
use crate::structs::store::Store;
use itertools::Itertools;

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


    // keeping last packet received per each sensor
    let unique_sensors_ids: Vec<String> = device_data.clone().into_iter().map(|datum| datum.sensor_id).unique().collect();
    let packets = unique_sensors_ids.iter().map(|sensor_id| device_data.iter().rfind(|datum| &datum.sensor_id == sensor_id));

    // TODO compute position regarding RSSI for each packet

    Ok(StatusCode::NOT_IMPLEMENTED.into_response())
}
