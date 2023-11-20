//! Receive commands via the [Adafruit Bluefruit LE UART Friend](https://learn.adafruit.com/introducing-the-adafruit-bluefruit-le-uart-friend?view=all)
//! bluetooth module (connected via UART) and print them.

#![deny(unsafe_code)]
#![forbid(unused)]
#![no_std]
#![no_main]

use panic_probe as _;

use defmt_rtt as _;

mod adafruit_bluefruit_le_uart_friend;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [EXTI1])]
mod app {
    use crate::adafruit_bluefruit_le_uart_friend::BluefruitLEUARTFriend;
    use stm32f4xx_hal::{pac, prelude::*, timer::MonoTimerUs, watchdog::IndependentWatchdog};

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimerUs<pac::TIM2>;

    #[shared]
    struct Shared {
        bt_module: BluefruitLEUARTFriend,
    }

    #[local]
    struct Local {
        watchdog: IndependentWatchdog,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
        let mono = ctx.device.TIM2.monotonic_us(&clocks);

        let gpioa = ctx.device.GPIOA.split();
        let gpiob = ctx.device.GPIOB.split();

        let bt_module = BluefruitLEUARTFriend::new(
            ctx.device.USART1,
            ctx.device.DMA2,
            gpiob.pb6,
            gpioa.pa10,
            &clocks,
        );

        // set up the watchdog
        let mut watchdog = IndependentWatchdog::new(ctx.device.IWDG);
        watchdog.start(500u32.millis());
        watchdog.feed();
        periodic::spawn().ok();

        defmt::info!("init done!");

        (
            Shared { bt_module },
            Local { watchdog },
            init::Monotonics(mono),
        )
    }

    /// Feed the watchdog to avoid hardware reset.
    #[task(priority=1, local=[watchdog])]
    fn periodic(ctx: periodic::Context) {
        defmt::trace!("feeding the watchdog!");
        ctx.local.watchdog.feed();
        periodic::spawn_after(100.millis()).ok();
    }

    #[task(binds = DMA2_STREAM2, shared = [bt_module])]
    fn bluetooth_dma_interrupt(mut ctx: bluetooth_dma_interrupt::Context) {
        defmt::debug!("received DMA2_STREAM2 interrupt (transfer complete)");
        ctx.shared.bt_module.lock(|bt_module| {
            bt_module.handle_bluetooth_message();
        });
    }

    #[task(binds = USART1, shared = [bt_module])]
    fn bluetooth_receive_interrupt(mut ctx: bluetooth_receive_interrupt::Context) {
        defmt::debug!("received USART1 interrupt (IDLE)");
        ctx.shared.bt_module.lock(|bt_module| {
            bt_module.handle_bluetooth_message();
        });
    }
}
