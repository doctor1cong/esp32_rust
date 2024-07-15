use esp_idf_hal::task::*;
use log::info;
use std::thread;

use crate::module::app_led::app_led_flash;

pub fn app_thread_start ()
{
    app_led_flash();
    info!("app thread start\r\n");
}