
#[derive(Debug, Clone)]
pub struct AdvertVideo {
    pub interest: i32,
    pub url: String, 
    pub length_sec: i32
}

#[derive(Debug, Clone, Copy)]
pub struct Tracker {
    pub id: i32,
    pub location: Option<i32>
}

#[derive(Debug, Clone, Copy)]
pub struct Receiver {
    pub id: i32,
    pub location: i32
}
