# projektgrupp17-iot-backend
Collects data from iot devices
Implemented in rust with rocket

![Rust](https://github.com/krummelur/projektgrupp17-iot-backend/workflows/Rust/badge.svg?branch=master&event=push)

# Build
* switch to nightly "rustup override set nightly"
* cargo run|build to run or build 

# Test
### Unittests, must not run in single thread.
cargo test --tests unittest
### Integration tests, must run in single thread
cargo test --tests integrationtest -- --test-threads=1

## endpoints
* /register/receiver_id/tag_id [POST]

* /register [POST]
```
{
loc: <receiver_id>,
tag: <rfid_tag_id>
}
```

registers that a specific device has been seen by a transceiver station, both the device and the station must exist.

* /trackers/tracker_id [GET]

Gets info about a specified tracker 

* /unregister/receiver_id/tracker_id [POST]
* /unregister [POST]

Unregisters a specific tracke from a specific receiver

* /videos/<display_id> [GET]

Gets the most relevant video for a specific display

* /views/<display_id>/<video_id>/<order_id>
```
{
length_seconds: <video_time_in_seconds>
}
```

Registers a video view. The video view is persisted in database, and credit are deducted from the order


## Environment
* RUST_IOT_ENVIRONMENT: PRODUCTION|TEST 
#### production
* SQL_USERNAME: username for database
* SQL_PASSWORD: password for database
* SQL_HOST: mysql host
* SQL_DB_NAME: database name
#### test
* SQL_USERNAME_TEST: username for test database
* SQL_PASSWORD_TEST: password for test database
* SQL_HOST_TEST: mysql test host
* SQL_DB_NAME_TEST: test database name

