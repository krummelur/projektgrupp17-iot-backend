
#[derive(Debug, Clone)]
pub struct AdvertVideo {
    pub interest: i32,
    pub url: String, 
    pub length_sec: i32
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub credits: i32, 
    pub user: String
}


#[derive(Debug, Clone)]
pub struct AdvertVideoOrder {
    pub video_id: i32,
    pub interest: i32,
    pub url: String, 
    pub length_sec: i32,
    pub order: String
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

#[derive(Debug, Clone, Copy)]
pub struct Display {
    pub id: i32,
    pub location: i32
}

#[derive(Debug, Clone)]
pub struct Agency {
    pub name: String,
    pub orgnr: String
}