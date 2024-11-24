#![no_std]
#![no_main]
#[allow(unused)]
use esp_backtrace as _;
use esp_hal::{
    analog::adc::{
        Adc,
        AdcConfig,
        AdcCalLine,
        AdcCalBasic,
        AdcCalCurve,
        Attenuation,
    }
    , dma::{
        Dma, 
        DmaPriority
    },  
    dma_circular_buffers, 
    gpio::{
        Io,
    }, 
    i2s::{
        DataFormat, 
        I2s, 
        I2sTx, 
        I2sWriteDma, 
        Standard
    }, peripherals::I2S0, peripherals::ADC1, prelude::*, time, xtensa_lx::timer::delay, Blocking
};

use crate::wave::constants::SINE;

mod wave;
mod oscilator;

const TX_BUFFER_SIZE: usize = 4096;
const STEP: f32 = 1.0;//(60.0 * 1024.0)/44100.0;
const STEP_DIV: f32 = SINE.len() as f32/44100.0;
const FREQ_DIV: f32 = 400.0/256.0;
// Se STEP aumenta, a frequencia aumenta tambem
// Se o número de amostras no seno aumenta a frequencia diminui

#[entry]
fn main() -> ! {
    
    // esp_println::logger::init_logger_from_env();
    // let delay = Delay::new();

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    
    //ADC setup
    let mut adc_config: AdcConfig<ADC1> = AdcConfig::new();
    let mut adc_pin = adc_config.enable_pin_with_cal::<_, AdcCalLine<_>>( 
        io.pins.gpio3,
        Attenuation::Attenuation11dB
    );


    let mut adc_driver = Adc::new(
        peripherals.ADC1,
        adc_config
    );

    
    //I2S and DMA setup
    let dma = Dma::new(peripherals.DMA);
    let dma_channel = dma.channel0;

    let (_, rx_descriptors, tx_buffer, tx_descriptors) = dma_circular_buffers!(0, TX_BUFFER_SIZE);

    let i2s = I2s::new(
        peripherals.I2S0,
        Standard::Philips,
        DataFormat::Data16Channel16,
        44100.Hz(),
        dma_channel.configure(false, DmaPriority::Priority0),
        rx_descriptors,
        tx_descriptors,
    );

    let mut i2s_tx: I2sTx<I2S0,Blocking> = i2s
        .i2s_tx
        .with_bclk(io.pins.gpio4)
        .with_ws(io.pins.gpio5)
        .with_dout(io.pins.gpio15)
        .build();




    let data = unsafe { 
        core::slice::from_raw_parts(&SINE as *const _ as *const u8, SINE.len() * 2) 
    };
    
    let mut step = 60.0 * STEP_DIV;
    let mut j = 0.0;
    for i in (0..tx_buffer.len()).step_by(2){
        let k = (j as usize) << 1;
        tx_buffer[i] = data[k];
        tx_buffer[i+1] = data[k + 1];
        j += step;

        if j >= (data.len()/2) as f32{ j = 0.0;}

    }


    let mut filler = [0u8; TX_BUFFER_SIZE];
    let mut transfer = i2s_tx.write_dma_circular(&tx_buffer).unwrap();
    

    let mut idx = 0.0;
    let transfer_size = transfer.available();
    for i in (0..transfer_size).step_by(2){
        let k = (idx as usize) << 1;
        filler[i] = data[k];
        filler[i+1] = data[k + 1];

        idx += step;
        if idx > (data.len()/2) as f32{
            idx = 0.0;
        }
    }

    transfer.push(&filler[0..transfer_size]).unwrap();
    
    let mut freq = adc_driver.read_blocking(&mut adc_pin) >> 4;
    let mut adc_counter: u32 = 0; 
    loop {
        adc_counter += 1;
        let avail = transfer.available();
        if avail > 0 {
            let avail = usize::min(10000, avail);
            for bidx in (0..avail).step_by(2) {
                // let k = (idx as usize) << 1;
                let bytes = SINE[idx as usize].to_ne_bytes();
                filler[bidx] = bytes[0];
                filler[bidx + 1] = bytes[1];
                idx += step;

                if idx >= (data.len()/2) as f32{
                    idx = 0.0;
                }
            }
            transfer.push(&filler[0..avail]).unwrap();
        }
        
        if adc_counter > 100_000{
           adc_counter = 0;
            let adc_read = adc_driver.read_blocking(&mut adc_pin) >> 4;
            let delta = abs(adc_read as i16 - freq as i16); 
                if delta > 7{
                    freq = adc_read;
                    step = (freq as f32) * FREQ_DIV * STEP_DIV;
                }
            // esp_println::println!("{freq}");
        }
        
    }
}


fn abs(x: i16) -> i16{
    if x < 0 {return -x;}
    return x;
}
