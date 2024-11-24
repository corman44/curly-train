use core::mem::take;

use embedded_hal::digital::StatefulOutputPin;
use rp_pico::hal::gpio::{FunctionSio, Pin, PullUp, SioOutput, ValidFunction};

pub struct DHT11<P>
where
    P: ValidFunction<FunctionSio<SioOutput>>
{
    /// Delay between Sensor Readings
    delay_ms: u32,
    /// Sig Pin to DHT11 board
    pin: Option<Pin<P, FunctionSio<SioOutput>, PullUp>>,
}

impl<P> DHT11<P>
where
    P: ValidFunction<FunctionSio<SioOutput>>
{
    pub fn new(pin: Pin<P, FunctionSio<SioOutput>, PullUp>, delay: u32) -> Self {
        Self {
            delay_ms: delay,
            pin: Some(pin),
        }
    }

    fn start_signal(&mut self) {
        if let Some(ref mut pin) = self.pin {
            let output_pin = pin.();

            // self.pin = Some(output_pin.into_push_pull_output());
        }
    }
}
