#[cfg(test)]
mod sensors_tests {
    use warp::hyper::body::HttpBody;
    use wifi_tracking_api::structs::sensors::{Sensor, Sensors};
    use wifi_tracking_api::controllers::sensors::{create_sensor, get_all_sensors, get_sensor_position};
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
        assert_eq!(result.unwrap().into_response().status(), 201);

        let sensors_bytes = get_all_sensors(sensors).await.unwrap()
            .into_response().into_body()
            .data().await.unwrap().unwrap();
        assert_eq!(sensors_bytes, "[\"test_sensor\"]");
    }

    #[tokio::test]
    async fn create_same_sensor_several_times() {
        let sensors = Sensors::new();
        let test_sensor = Sensor {
            id: "test_sensor".to_string(),
            pos: Position {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        };

        let result = create_sensor(test_sensor.clone(), sensors.clone()).await;
        assert_eq!(result.unwrap().into_response().status(), 201);
        let result = create_sensor(test_sensor.clone(), sensors.clone()).await;
        assert_eq!(result.unwrap().into_response().status(), 200);
        let result = create_sensor(test_sensor.clone(), sensors.clone()).await;
        assert_eq!(result.unwrap().into_response().status(), 200);

        let sensors_bytes = get_all_sensors(sensors).await.unwrap()
            .into_response().into_body()
            .data().await.unwrap().unwrap();
        assert_eq!(sensors_bytes, "[\"test_sensor\"]");
    }

    #[tokio::test]
    async fn get_sensor_location() {
        let position = Position {
            x: 1.23,
            y: 4.56,
            z: 7.89
        };

        // creating sensor
        let sensors = Sensors::new();
        let test_sensor = Sensor {
            id: "test_sensor".to_string(),
            pos: position.clone()
        };
        create_sensor(test_sensor.clone(), sensors.clone()).await.ok();

        // getting sensor position
        let position_bytes = get_sensor_position(sensors, test_sensor.id).await
            .unwrap().into_response().into_body().data().await
            .unwrap().unwrap();
        let retr_pos: Position = serde_json::from_str(&*String::from_utf8(position_bytes.to_vec()).unwrap()).unwrap();
        assert_eq!(position, retr_pos);
    }

    #[tokio::test]
    async fn get_non_existent_sensor_location() {
        let sensors = Sensors::new();
        let response = get_sensor_position(sensors, "this_is_not_a_sensor".to_string()).await;
        assert_eq!(response.is_ok(), true);
        assert_eq!(response.unwrap().into_response().status(), 404);
    }
}
