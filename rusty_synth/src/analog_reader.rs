use esp_hal::{
    analog::adc::{Adc, AdcCalLine, AdcChannel, AdcConfig, AdcPin, Attenuation},
    gpio::AnalogPin,
    peripherals::ADC1,
};

pub struct AnalogReader<PIN: AdcChannel + AnalogPin, F: Fn(u16) -> f32> {
    adc_pin: AdcPin<PIN, ADC1, AdcCalLine<ADC1>>,
    activation: F,
    accumulator: u16,
}

impl<PIN, F> AnalogReader<PIN, F>
where
    PIN: AdcChannel + AnalogPin,
    F: Fn(u16) -> f32,
{
    pub fn new(pin: PIN, activation: F, adc_config: &mut AdcConfig<ADC1>) -> AnalogReader<PIN, F> {
        let adc_pin =
            adc_config.enable_pin_with_cal::<PIN, AdcCalLine<_>>(pin, Attenuation::Attenuation11dB);
        AnalogReader {
            adc_pin,
            activation,
            accumulator: 0,
        }
    }
    pub fn read(&mut self, driver: &mut Adc<ADC1>) -> f32 {
        let value = driver.read_blocking(&mut self.adc_pin);
        //exponential moving average (smoothing / (window_size + 1))
        let alpha = 2.0 / 65.0;
        self.accumulator =
            ((alpha * value as f32) + (1.0 - alpha) * self.accumulator as f32) as u16;
        (self.activation)(self.accumulator)
    }
}
