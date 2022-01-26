#[cfg(test)]
mod sensors_tests {
    use warp::hyper::body::HttpBody;
    use wifi_tracking_api::structs::sensors::Sensors;
    use wifi_tracking_api::controllers::sensors::get_all_sensors;
    use warp::reply::Reply;

    #[tokio::test]
    async fn empty_sensors_at_instantiation() {
        let result = get_all_sensors(Sensors::new()).await;
        assert_eq!(result.is_ok(), true);

        let bytes = result
            .unwrap().into_response().into_body()
            .data().await.unwrap().unwrap();
        assert_eq!(bytes.len(), 2); // == []
    }
}
