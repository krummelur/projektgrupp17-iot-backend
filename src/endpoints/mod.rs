pub mod catchers;
pub mod default;
pub mod iot_devices;
pub mod videos;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterBody {
    loc: String,
    tag: String
}
