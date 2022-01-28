#[cfg(test)]
mod distance_tests {
    use warp::hyper::body::HttpBody;
    use warp::hyper::StatusCode;
    use wifi_tracking_api::structs::sensors::{Sensor, Sensors};
    use warp::reply::Reply;
    use wifi_tracking_api::controllers::distance::get_device_pos;
    use wifi_tracking_api::structs::store::Store;

    #[tokio::test]
    async fn get_distance_with_no_sensors() {
        let sensors = Sensors::new();
        let store = Store::new();
        let device_id = "9a:8f:99:87:0d:62".to_string();
        let expected = format!("No data was reported for device {}.", device_id);

        let result = get_device_pos(sensors, store, device_id).await;
        assert_eq!(result.is_ok(), true);

        let response = result.unwrap().into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(response.into_body().data().await.unwrap().unwrap(), expected);
    }
}
