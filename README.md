# Wi-Fi tracking API

This application received data from Wi-Fi sniffers, and computes positions with RSSI triangulation algorithm.


## Build and run 

```shell
cargo build
cargo run
```

## Test commands

```shell
## Get sensors
curl --location --request GET '127.0.0.1:3030/v1/sensors'

## Create sensor
curl --location --request POST '127.0.0.1:3030/v1/sensors' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "id": "thisisatestsensor",
    "pos": {
        "x": 55.24558856,
        "y": 3.1458965,
        "z": 12.4269
    }
}'
```