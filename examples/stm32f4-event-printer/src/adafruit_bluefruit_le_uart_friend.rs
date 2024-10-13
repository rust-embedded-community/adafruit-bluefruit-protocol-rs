use stm32f4xx_hal::dma::config::DmaConfig;
use stm32f4xx_hal::dma::{PeripheralToMemory, Stream2, StreamsTuple, Transfer};
use stm32f4xx_hal::gpio::{PA10, PB6};
use stm32f4xx_hal::pac::{DMA2, USART1};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::rcc::Clocks;
use stm32f4xx_hal::serial;
use stm32f4xx_hal::serial::{Rx, Serial};

pub type USART1RxBufferInt =
    &'static mut [u8; adafruit_bluefruit_protocol::MAX_CONTROLLER_MESSAGE_LENGTH];
pub type USART1RxBuffer = Option<USART1RxBufferInt>;
pub type USART1RxTransfer =
    Transfer<Stream2<DMA2>, 4_u8, Rx<USART1>, PeripheralToMemory, USART1RxBufferInt>;

/// Represents an [Adafruit Bluefruit LE UART Friend](https://learn.adafruit.com/introducing-the-adafruit-bluefruit-le-uart-friend) connected to an STM32F4 chip.
pub struct BluefruitLEUARTFriend {
    pub rx_transfer: USART1RxTransfer,
    pub rx_buffer: USART1RxBuffer,
}

impl BluefruitLEUARTFriend {
    /// set up the Adafruit Bluefruit UART LE Friend connected on PB6 & PA10
    ///
    /// note: it will use DMA for the UART connection, the corresponding interrupt must be handled.
    ///
    /// TODO: get rid of all stm32f4xx_hal references, use generic embedded-hal traits!
    pub fn new(
        pac_usart1: USART1,
        pac_dma2: DMA2,
        tx_pin: PB6,
        rx_pin: PA10,
        clocks: &Clocks,
    ) -> BluefruitLEUARTFriend {
        let usart1 = Serial::new(
            pac_usart1,
            (tx_pin.into_alternate(), rx_pin.into_alternate()),
            serial::Config::default()
                .baudrate(9600.bps())
                .dma(serial::config::DmaConfig::Rx),
            clocks,
        )
        .expect("USART1 can be set up");

        let (_usart1_tx, mut usart1_rx) = usart1.split();
        usart1_rx.listen_idle();

        let streams = StreamsTuple::new(pac_dma2);
        let rx_stream = streams.2;
        let rx_buffer = cortex_m::singleton!(: [u8; adafruit_bluefruit_protocol::MAX_CONTROLLER_MESSAGE_LENGTH] = [0; adafruit_bluefruit_protocol::MAX_CONTROLLER_MESSAGE_LENGTH])
            .expect("RX buffer singleton created");
        let mut rx_transfer = Transfer::init_peripheral_to_memory(
            rx_stream,
            usart1_rx,
            rx_buffer,
            None,
            DmaConfig::default()
                .memory_increment(true)
                .fifo_enable(true)
                .fifo_error_interrupt(true)
                .transfer_complete_interrupt(true),
        );
        rx_transfer.start(|_rx| {});
        let rx_buffer = cortex_m::singleton!(: [u8; adafruit_bluefruit_protocol::MAX_CONTROLLER_MESSAGE_LENGTH] = [0; adafruit_bluefruit_protocol::MAX_CONTROLLER_MESSAGE_LENGTH])
            .expect("RX buffer singleton created");

        BluefruitLEUARTFriend {
            rx_transfer,
            rx_buffer: Some(rx_buffer),
        }
    }

    pub fn handle_bluetooth_message(&mut self) {
        if !self.rx_transfer.is_transfer_complete() {
            return;
        }

        let (filled_buffer, _) = self
            .rx_transfer
            .next_transfer(self.rx_buffer.take().unwrap())
            .unwrap();
        defmt::debug!(
            "bluetooth: DMA transfer complete, received {:a}",
            filled_buffer
        );

        let parser = adafruit_bluefruit_protocol::Parser::new(filled_buffer);
        for event in parser {
            defmt::info!("received event over bluetooth: {:?}", &event);
        }

        // switch out the buffers
        filled_buffer.fill(0);
        self.rx_buffer = Some(filled_buffer);

        self.rx_transfer.clear_idle_interrupt();
    }
}
