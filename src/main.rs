#![no_std]
#![no_main]

extern crate panic_semihosting;

use embedded_hal::digital::OutputPin;
use rtfm::{app, Instant};
use stm32f3xx_hal::{
    gpio::{gpioc, GpioExt, Output, PushPull},
    rcc::RccExt,
};

const PERIOD: u32 = 2_000_000;

#[app(device = stm32f3xx_hal::stm32)]
const APP: () = {
    static mut LED: gpioc::PC13<Output<PushPull>> = ();

    #[init(schedule = [led_on])]
    fn init() -> init::LateResources {
        let mut rcc = device.RCC.constrain();

        let mut gpioc = device.GPIOC.split(&mut rcc.ahb);
        let led = gpioc
            .pc13
            .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);

        schedule.led_on(Instant::now()).unwrap();

        init::LateResources { LED: led }
    }

    #[task(schedule = [led_off], resources = [LED])]
    fn led_on() {
        resources.LED.set_high();
        schedule.led_off(scheduled + PERIOD.cycles()).unwrap();
    }

    #[task(schedule = [led_on], resources = [LED])]
    fn led_off() {
        resources.LED.set_low();
        schedule.led_on(scheduled + PERIOD.cycles()).unwrap();
    }

    extern "C" {
        fn USART1_EXTI25();
    }
};
