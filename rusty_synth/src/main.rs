
/*======================================= IMPORTS =======================================*/ 
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
    },
    dma::{
        Dma, 
        DmaPriority
    },  
    i2s::{
        DataFormat, 
        I2s, 
        I2sTx, 
        I2sWriteDma, 
        Standard
    }, 
    peripherals::{
        I2S0,
        ADC1
    },
    dma_circular_buffers, 
    gpio::Io,
    prelude::*, 
    Blocking,
};

use crate::wave::constants::SINE;
use crate::oscilator::{
    WaveForm,
    Oscilator,
};

/*======================================= MODULES =======================================*/ 

mod wave;
mod oscilator;

/*======================================= CONSTANTS =======================================*/ 

const TX_BUFFER_SIZE: usize = 4096;
const STEP: f32 = 1.0;//(60.0 * 1024.0)/44100.0;
const STEP_DIV: f32 = SINE.len() as f32/44100.0;
const FREQ_DIV: f32 = 400.0/256.0;
// Se STEP aumenta, a frequencia aumenta tambem
// Se o número de amostras no seno aumenta a frequencia diminui

/*======================================= MAIN =======================================*/ 

#[entry]
fn main() -> ! {
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
    let mut signal_buffer = [0i16; TX_BUFFER_SIZE];
    let mut oscilator = Oscilator::new(
        60.0,
        60.0,
        10_000.0,
        WaveForm::Triangle
    );

    oscilator.gen_signal(
        &mut signal_buffer,
        TX_BUFFER_SIZE/2,
        true
    );



    copy_bytes(&signal_buffer, tx_buffer, TX_BUFFER_SIZE);
    let mut transfer = i2s_tx.write_dma_circular(&tx_buffer).unwrap();
    let mut filler = [0u8; TX_BUFFER_SIZE];

    let transfer_size = transfer.available();
    oscilator.gen_signal(&mut signal_buffer, 
        transfer_size/2, 
        true
    );
    copy_bytes(&signal_buffer, &mut filler, transfer_size);

    transfer.push(&filler[0..transfer_size]).unwrap();
    
    let mut freq = adc_driver.read_blocking(&mut adc_pin) >> 4;
    let mut adc_counter: u32 = 0;
    oscilator.set_frequency(freq as f32 * FREQ_DIV);
    loop {
        adc_counter += 1;
        let avail = transfer.available();
        if avail > 0 {
            let avail = usize::min(10000, avail);
            oscilator.gen_signal(&mut signal_buffer, avail/2, true);
            copy_bytes(&signal_buffer, &mut filler, avail);
            transfer.push(&filler[0..avail]).unwrap();
        }
        
        if adc_counter > 100_000{
           adc_counter = 0;
            let adc_read = adc_driver.read_blocking(&mut adc_pin) >> 4;
            let delta = abs(adc_read as i16 -  freq as i16); 
            // esp_println::println!("ADC READ = {adc_read}");
                if delta > 7{
                    oscilator.set_frequency(adc_read as f32 * FREQ_DIV);
                    freq = adc_read;
                }
        }
        
    }
}

fn copy_bytes(signal_buffer: &[i16], filler: &mut[u8], size: usize){
    let signal_buffer = unsafe{
        core::slice::from_raw_parts(
            signal_buffer as *const _ as *const u8,
            signal_buffer.len() * 2
        )
    };

    filler[..size].copy_from_slice(&signal_buffer[..size]);
}

fn abs(x: i16) -> i16{
    if x < 0 {return -x;}
    return x;
}
