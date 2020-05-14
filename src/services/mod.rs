/**
 * Rfid device business logic
 */
pub mod devices;
/**
 * Videos business logic
 */
pub mod videos;

#[derive(Debug)]
pub enum DeviceServiceError {
    NoSuchTracker,
    NoSuchReceiver,
    Other
}


#[derive(Debug)]
pub enum VideoServiceError {
    NoSuchVideo,
    NoSuchDisplay,
    NoSuchOrder,
    NoSuchDisplayLocation,
    Other,
}