#[cfg(test)]
mod sensors_tests {
    use warp::hyper::body::HttpBody;
    use wifi_tracking_api::structs::sensors::{Sensor, Sensors};
    use wifi_tracking_api::controllers::sensors::{create_sensor, get_all_sensors};
    use warp::reply::Reply;
    use wifi_tracking_api::structs::position::Position;

    #[tokio::test]
    async fn empty_sensors_at_instantiation() {
        let result = get_all_sensors(Sensors::new()).await;
        assert_eq!(result.is_ok(), true);

        let bytes = result
            .unwrap().into_response().into_body()
            .data().await.unwrap().unwrap();
        assert_eq!(bytes.len(), 2); // == []
    }

    #[tokio::test]
    async fn create_new_sensor() {
        let sensors = Sensors::new();
        let test_sensor = Sensor {
            id: "test_sensor".to_string(),
            pos: Position {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        };

        let result = create_sensor(test_sensor, sensors.clone()).await;
        assert_eq!(result.unwrap().into_response().status(), 200);

        let sensors_bytes = get_all_sensors(sensors).await.unwrap()
            .into_response().into_body()
            .data().await.unwrap().unwrap();
        assert_eq!(sensors_bytes, "[\"test_sensor\"]");
    }
}
