#[cfg(test)]
mod store_tests {
    use wifi_tracking_api::controllers::wifi_data::save_wifi_datum;
    use wifi_tracking_api::structs::data::Datum;
    use wifi_tracking_api::structs::store::Store;

    #[tokio::test]
    async fn get_device_data() {
        let store = Store::new();
        let device_id = "04-26-24-44-F3-B0".to_string();

        let datum1 = Datum {
            sensor_id: "59-2A-7F-97-E1-B9".to_string(),
            sender_mac: device_id.clone(),
            receiver_mac: "07-43-A5-AB-2D-1A".to_string(),
            rssi: -42,
            timestamp: 1643382243511
        };
        let datum2 = Datum {
            sensor_id: "48-18-74-4E-4D-84".to_string(),
            sender_mac: device_id.clone(),
            receiver_mac: "07-43-A5-AB-2D-1A".to_string(),
            rssi: -40,
            timestamp: 1643382244269
        };
        let datum3 = Datum {
            sensor_id: "59-2A-7F-97-E1-B9".to_string(),
            sender_mac: device_id.clone(),
            receiver_mac: "07-43-A5-AB-2D-1A".to_string(),
            rssi: -65,
            timestamp: 1643382345511
        };

        save_wifi_datum(datum1.clone(), store.clone()).await.ok();
        save_wifi_datum(datum2.clone(), store.clone()).await.ok();
        save_wifi_datum(datum3.clone(), store.clone()).await.ok();

        // this datum that does not belong to device_id
        save_wifi_datum(Datum {
            sensor_id: "59-2A-7F-97-E1-B9".to_string(),
            sender_mac: "CE-BA-00-9A-9B-AD".to_string(),
            receiver_mac: "07-43-A5-AB-2D-1A".to_string(),
            rssi: -42,
            timestamp: 1643382345511
        }, store.clone()).await.ok();

        let data = store.get_device_data(device_id);
        assert_eq!(data.len(), 3);
        assert!(data.contains(&datum1));
        assert!(data.contains(&datum2));
        assert!(data.contains(&datum3));
    }
}
