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
    

	let mut pin2 = pins.d2.into_output();
    let mut pin13 = pins.d13.into_output();
    loop {
        for i in 0..60
            {
                println!("{}", i);
                pin2.toggle();
                arduino_hal::delay_ms(1000);
                pin13.toggle();
            }
        println!("Minute");
    }
}
