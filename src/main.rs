#![no_std]
#![no_main]

use panic_rtt_target as _;

use cortex_m_rt::entry;
use fugit::HertzU32;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::pac::{I2C1, Peripherals, RCC};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::rcc::{Enable, Reset};

#[entry]
unsafe fn main() -> ! {
    rtt_init_print!();

    let mut dp = Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(HertzU32::MHz(8))
        .sysclk(HertzU32::MHz(72))
        .hclk(HertzU32::MHz(72))
        .pclk1(HertzU32::MHz(36))
        .pclk2(HertzU32::MHz(72))
        .freeze(&mut flash.acr);

    I2C1::enable(&*RCC::ptr());
    I2C1::reset(&*RCC::ptr());

    dp.I2C1
        .cr2
        .modify(|_, register| unsafe { register.freq().bits(clocks.pclk1().to_MHz() as u8) });

    dp.I2C1.cr1.modify(|_, register| register.pe().clear_bit());

    let freq = 100_000u32;

    dp.I2C1
        .ccr
        .write(|register| unsafe { register.ccr().bits(((clocks.pclk1().to_Hz() / (freq * 2)) as u16).max(4)) });

    let trise = (1000f32 / (1f32 / clocks.pclk1().to_MHz() as f32)) as u8;
    let trise = 9;
    rprintln!("trise {}", trise);
    dp.I2C1.trise.write(|register| register.trise().bits(trise));

    dp.I2C1.cr1.modify(|_, register| register.pe().set_bit());

    rprintln!("CONSUME CPU CYCLESSSSSS");
    rprintln!("CONSUME CPU CYCLESSSSSS");
    rprintln!("CONSUME CPU CYCLESSSSSS");
    rprintln!("CONSUME CPU CYCLESSSSSS");
    rprintln!("CONSUME CPU CYCLESSSSSS");

    dp.I2C1.cr1.modify(|_, register| register.start().set_bit());
    rprintln!("Started");

    while dp.I2C1.sr1.read().sb().bit_is_clear(){
        dp.I2C1.sr2.read();
    }
    rprintln!("Started");
    loop {}
}
