use warp::http;
use crate::structs::{Datum, Store};
use warp::{Filter};

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

pub fn json_body() -> impl Filter<Extract = (Datum,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}