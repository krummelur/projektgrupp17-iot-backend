# projektgrupp17-iot-backend
Collects data from iot devices
Implemented in rust with rocket

![Rust](https://github.com/krummelur/projektgrupp17-iot-backend/workflows/Rust/badge.svg?branch=master&event=push)

# Docs
### [Full documentation here](https://krummelur.github.io/projektgrupp17-iot-backend/doc/proj17_iot_server/index.html)

# Build
* switch to nightly "rustup override set nightly"
* cargo run|build to run or build 

# Test
### Unittests, must not run in single thread.
cargo test --tests unittest
### Integration tests, must run in single thread
cargo test --tests integrationtest -- --test-threads=1

## endpoints
See [documentation](https://krummelur.github.io/projektgrupp17-iot-backend/doc/proj17_iot_server/index.html)

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

