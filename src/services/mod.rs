/**
 * This module contains the business logic layer of the application
 */
pub mod devices;
pub mod videos;

#[derive(Debug)]
pub enum DeviceServiceError {
    NoSuchTracker,
    NoSuchReceiver
}


#[derive(Debug)]
pub enum VideoServiceError {
    NoSuchVideo,
    NoSuchDisplay,
    NoSuchOrder,
    NoSuchDisplayLocation,
    Other,
}