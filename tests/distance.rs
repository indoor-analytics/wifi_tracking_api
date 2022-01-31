#[cfg(test)]
mod distance_tests {
    use warp::hyper::body::HttpBody;
    use warp::hyper::StatusCode;
    use wifi_tracking_api::structs::sensors::{Sensor, Sensors};
    use wifi_tracking_api::structs::position::Position;
    use warp::reply::Reply;
    use wifi_tracking_api::controllers::distance::get_device_pos;
    use wifi_tracking_api::controllers::sensors::create_sensor;
    use wifi_tracking_api::structs::data::Datum;
    use wifi_tracking_api::structs::store::Store;

    #[tokio::test]
    async fn get_distance_with_no_data() {
        let sensors = Sensors::new();
        let sensor = Sensor {
            id: "azdazd".to_string(),
            pos: Position {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        };
        create_sensor(sensor, sensors.clone()).await.ok();
        let store = Store::new();
        let device_id = "9a:8f:99:87:0d:62".to_string();
        let expected = format!("No data was reported for device {}.", device_id);

        let result = get_device_pos(sensors, store, device_id).await;
        assert_eq!(result.is_ok(), true);

        let response = result.unwrap().into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(response.into_body().data().await.unwrap().unwrap(), expected);
    }

    #[tokio::test]
    async fn get_distance_with_no_sensors() {
        let sensors = Sensors::new();
        let store = Store::new();
        let device_id = "9a:8f:99:87:0d:62".to_string();

        let result = get_device_pos(sensors, store, device_id).await;
        assert_eq!(result.is_ok(), true);

        let response = result.unwrap().into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);
        assert_eq!(response.into_body().data().await.unwrap().unwrap(), "No sensors have been registered yet.");
    }

    #[tokio::test]
    async fn get_device_position() {
        let device_id = "9a:8f:99:87:0d:62".to_string();

        let sensors = Sensors::new();
        let sensor1_id = "sensor1".to_string();
        sensors.sensors.write().insert(sensor1_id.clone(), Sensor {
            id: sensor1_id.clone(),
            pos: Position {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        });
        let sensor2_id = "sensor2".to_string();
        sensors.sensors.write().insert(sensor2_id.clone(), Sensor {
            id: sensor2_id.clone(),
            pos: Position {
                x: 2.0,
                y: 4.0,
                z: 3.0
            }
        });
        let store = Store::new();

        // simulate device being sniffed by two sensors
        store.data.write().push_back(Datum {
            sensor_id: sensor1_id.clone(),
            sender_mac: device_id.clone(),
            receiver_mac: "not_important".to_string(),
            rssi: 0,
            timestamp: 0
        });
        store.data.write().push_back(Datum {
            sensor_id: sensor2_id.clone(),
            sender_mac: device_id.clone(),
            receiver_mac: "not_important".to_string(),
            rssi: 0,
            timestamp: 0
        });

        let result = get_device_pos(sensors, store, device_id.clone()).await;
        assert_eq!(result.is_ok(), true);

        let response = result.unwrap().into_response();
        assert_eq!(response.status(), StatusCode::OK);

        // getting device position
        let position_bytes = response.into_body().data().await.unwrap().unwrap();
        let position: Position = serde_json::from_str(&*String::from_utf8(position_bytes.to_vec()).unwrap()).unwrap();
        println!("{}", position);
    }
}
