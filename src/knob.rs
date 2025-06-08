use crate::*;

pub type Adc = saadc::Saadc<'static, 1>;

pub struct Knob(Adc);

impl Knob {

    /// Creates a new `Knob` instance with the provided ADC configuration.
    /// It calibrates the ADC before returning the instance.
    /// # Arguments
    /// * `adc` - An instance of the ADC to be used for measuring the knob's position.
    /// # Returns
    /// A new instance of the `Knob` struct, which wraps the ADC.
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await;
        Self(adc)
    }

    /// Measures the position of the knob using the ADC.
    /// It reads a sample from the ADC, scales it, and maps it to a range of levels.
    /// # Returns
    /// A `u32` value representing the knob's position, scaled to a range from 0 to `LEVELS - 1`.
    pub async fn measure(&mut self) -> u32 {
        let mut buf = [0];
        self.0.sample(&mut buf).await;
        let raw = buf[0].clamp(0, 0x7fff) as u16;
        let scaled = raw as f32 / 10_000.0;
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32
    }
}
