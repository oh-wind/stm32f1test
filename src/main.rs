#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; // MUST NEEDED
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn f103_main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    rcc.cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    let mut i = 0usize;
    const ONE_SECS: usize = (72 * 1_000_000) / 2;
    loop {
        if i == ONE_SECS {
            led.toggle();
            i = 0;
        } else {
            i = i + 1;
        }
    }
}
