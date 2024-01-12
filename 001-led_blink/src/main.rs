#![no_std]
#![no_main]

use core::cell::RefCell;
use avr_device::interrupt;
use panic_halt as _;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> = interrupt::Mutex::new(RefCell::new(None));

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(
            |cs|
                {
                    if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut()
                        {
                            let _ = ufmt::uwriteln!(console, $($t)*);
                        }
                }
        )
        
    };
}

fn put_console(console:Console)
    {
        interrupt::free(
            |cs|
                {
                    *CONSOLE.borrow(cs).borrow_mut() = Some(console);
                }
        )
    }
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);
    println!("Hello World");
    

	let mut led02 = pins.d2.into_output();
	let mut led03 = pins.d3.into_output();
    let mut led04 = pins.d4.into_output();
    let mut led05 = pins.d5.into_output();
    let mut led06 = pins.d6.into_output();
    let mut led07 = pins.d7.into_output();
    //let mut led08 = pins.d8.into_output();
    let mut led09 = pins.d9.into_output();
    let mut led10 = pins.d10.into_output();
    let mut led11 = pins.d11.into_output();
    let mut led12 = pins.d12.into_output();
    let mut led13 = pins.d13.into_output();

    loop {
    	led02.toggle();
        arduino_hal::delay_ms(50);
        led02.toggle();
        led03.toggle();
        arduino_hal::delay_ms(50);
        led03.toggle();
        led04.toggle();
        arduino_hal::delay_ms(50);
        led04.toggle();
        led05.toggle();
        arduino_hal::delay_ms(50);
        led05.toggle();
        led06.toggle();
        arduino_hal::delay_ms(50);
        led06.toggle();
        led07.toggle();
        arduino_hal::delay_ms(50);
        led07.toggle();
        led09.toggle();
        arduino_hal::delay_ms(50);
        led09.toggle();
        led10.toggle();
        arduino_hal::delay_ms(50);
        led10.toggle();
        led11.toggle();
        arduino_hal::delay_ms(50);
        led11.toggle();
        led12.toggle();
        arduino_hal::delay_ms(50);
        led12.toggle();
        led13.toggle();
        arduino_hal::delay_ms(50);
        led13.toggle();
        led12.toggle();
        arduino_hal::delay_ms(50);
        led12.toggle();
        led11.toggle();
        arduino_hal::delay_ms(50);
        led11.toggle();
        led10.toggle();
        arduino_hal::delay_ms(50);
        led10.toggle();
        led09.toggle();
        arduino_hal::delay_ms(50);
        led09.toggle();
        led07.toggle();
        arduino_hal::delay_ms(50);
        led07.toggle();
        led06.toggle();
        arduino_hal::delay_ms(50);
        led06.toggle();
        led05.toggle();
        arduino_hal::delay_ms(50);
        led05.toggle();
        led04.toggle();
        arduino_hal::delay_ms(50);
        led04.toggle();
        led03.toggle();
        arduino_hal::delay_ms(50);
        led03.toggle();
    }
}
