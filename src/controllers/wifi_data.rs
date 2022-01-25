use warp::http;
use crate::structs::{Datum, Store};

pub async fn save_wifi_datum(
    datum: Datum,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    store.data.write().push_back(datum);
    Ok(warp::reply::with_status(
        "Received",
        http::StatusCode::OK,
    ))
}
