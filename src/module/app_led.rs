use log::info;

use esp_idf_svc::http::status::OK;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

pub fn app_led_flash ()
{
    let peripherals = Peripherals::take().unwrap();
    /*************************************** esp gpio ***************************************/      
    let handle_led = thread::spawn(|| -> Result<(), EspError> {
        let mut led_r= PinDriver::output(peripherals.pins.gpio9)?;
        let mut led_g= PinDriver::output(peripherals.pins.gpio10)?;
        loop{ 
                led_r.set_high()?;
                led_g.set_high()?;
                info!("led rg high");
                FreeRtos::delay_ms(600);
                led_r.set_low()?;
                led_g.set_low()?;
                info!("led rg low");
                FreeRtos::delay_ms(600);

                info!("[led thread:{:?}] ",thread::current().id());
        }
    });
}