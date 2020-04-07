# projektgrupp17-iot-backend
Collects data from iot devices
Implemented in rust with rocket
## endpoints
* /register/station_id/tracker_id (post)

registers that a specific device has been seen by a transceiver station, both the device and the station must exist.

# build
* switch to nightly "rustup override set nightly"
* cargo run|build|test to run build or test
