pub mod error;
pub mod network;
pub mod ffi; // اضافه شدن ماژول رابط کاربری

// ماژول‌های موقت (اگر فایل‌هایشان را هنوز نساخته‌اید)
pub mod router {
    pub mod pid {
        pub struct RouteWeightController;
        impl RouteWeightController {
            pub fn new(_p: f64, _i: f64, _d: f64, _seed: u64) -> Self { Self }
        }
    }
}
// pub mod judge; 

pub use error::{AeonError, Result};
