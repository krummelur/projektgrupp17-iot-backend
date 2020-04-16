# projektgrupp17-iot-backend
Collects data from iot devices
Implemented in rust with rocket

![Rust](https://github.com/krummelur/projektgrupp17-iot-backend/workflows/Rust/badge.svg?branch=master&event=push)

## endpoints
* /register/receiver_id/tracker_id (post)

registers that a specific device has been seen by a transceiver station, both the device and the station must exist.

* /trackers/tracker_id (get)

Unregisters a specific tracke from a specific receiver

* /videos/<display_id> (get)

Gets the most relevant video for a specific display


# build
* switch to nightly "rustup override set nightly"
* cargo run|build|test to run build or test

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

