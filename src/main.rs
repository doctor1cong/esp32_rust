#![allow(unused)]
 
use std::fmt::format;
use std::fmt::Write;
use std::string;
// use std::slice::Join;
use std::time::Duration;

use esp_idf_svc::http::status::OK;
use log::info;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart;
use esp_idf_hal::delay::BLOCK; 
use esp_idf_hal::sys::EspError;

pub mod module;



fn main()->anyhow::Result<()> {
    /* sys init */
    // It is necessary to call this function once. Otherwise some patches to the runtime
    esp_idf_svc::sys::link_patches();// implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::log::EspLogger::initialize_default();// Bind the log crate to the ESP Logging facilities
    log::info!("Hello, world!");

    // let peripherals = Peripherals::take().unwrap();
    // /*************************************** esp gpio ***************************************/      
    // let handle_led = thread::spawn(|| -> Result<(), EspError> {
    //     let mut led_r= PinDriver::output(peripherals.pins.gpio9)?;
    //     let mut led_g= PinDriver::output(peripherals.pins.gpio10)?;
    //     loop{ 
    //             led_r.set_high()?;
    //             led_g.set_high()?;
    //             info!("led rg high");
    //             FreeRtos::delay_ms(600);
    //             led_r.set_low()?;
    //             led_g.set_low()?;
    //             info!("led rg low");
    //             FreeRtos::delay_ms(600);

    //             info!("[led thread:{:?}] ",thread::current().id());
    //     }
    // });

    
    /*************************************** thread ***************************************/
    let handle_test = thread::spawn(|| {
        for i in 1..20 {
            info!("[test thread:{:?}] num:{}",thread::current().id(),i);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    /*************************************** esp usart ***************************************/
    let handle_usart = thread::spawn(|| {
        let config = uart::config::Config::default().baudrate(Hertz(115_200));
        let mut uart1: uart::UartDriver = uart::UartDriver::new(
            peripherals.uart1,
            peripherals.pins.gpio1,
            peripherals.pins.gpio2,
            Option::<AnyIOPin>::None,
            Option::<AnyIOPin>::None,
            &config
        ).unwrap();
        let mut i:u32 = 0;
        let mut data_arr:u8 =  {5;20};
        let mut str:&str =  "abcdefg\r\n";
        uart1.write_str(&str);
        info!("[usart thread:{:?}]",thread::current().id());
        loop {
            
            let mut buf = [0_u8; 1];
            let mut buf_len = buf.len();
            let mut index = 0;
            uart1.read(&mut buf,BLOCK);
            uart1.write_char(buf[0] as char);
            if buf [0] == '\r' as u8{
                uart1.write_char('\n');
                uart1.write_str("Thank you!\r\n");
                info!("[usart thread:{:?}]",thread::current().id());
            }
        }
    });

    loop {
        info!("[main thread:{:?}]",thread::current().id());
        thread::sleep(Duration::from_millis(2000));
    }

    handle_led.join().unwrap();
    handle_test.join().unwrap();
    handle_usart.join().unwrap();
    // return anyhow::Result<()> ;
}
