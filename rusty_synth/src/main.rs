//! This shows how to use the TIMG peripheral interrupts.
//!
//! There is TIMG0 which contains a general purpose timer and a watchdog timer.

//% CHIPS: esp32 esp32c2 esp32c3 esp32c6 esp32h2 esp32s2 esp32s3

#![no_std]
#![no_main]


use esp_backtrace as _;
use esp_hal::{
    dma::{Dma, DmaPriority},  dma_circular_buffers, gpio::{Io, Level, Output}, i2s::{DataFormat, I2s, I2sTx, I2sWrite, I2sWriteDma, Standard}, peripherals::I2S0, prelude::*, Blocking
};
use wave::sin_i16_bytes;

mod wave;

use crate::wave::{
    sin_i16,
    constants::{
        FULL_WAVE,
        SINE
    }
};

static mut PHASE: f32 = 0.0;


const TX_BUFF_LEN: usize = 4096;
// const STEP: f32 = (60.0 * FULL_WAVE as f32)/44100.0;
const STEP: f32 = 1.0;


#[entry]
fn main() -> ! {
    
    let mut sys_cfg = esp_hal::Config::default();
    sys_cfg.cpu_clock = CpuClock::max();

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    
    
    
    //DMA init 
    let dma = Dma::new(peripherals.DMA);
    let dma_channel = dma.channel0;
    
    let (_, rx_desc, tx_buff, tx_desc) = dma_circular_buffers!(0, TX_BUFF_LEN);
    //I2s init
    let i2s = I2s::new(
        peripherals.I2S0,
        Standard::Philips,
        DataFormat::Data16Channel16,
        44100.Hz(),
        dma_channel.configure(false, DmaPriority::Priority0),
        rx_desc,
        tx_desc,
    );



    let mut i2s_tx: I2sTx<I2S0, Blocking> = i2s 
        .i2s_tx
        .with_bclk(io.pins.gpio15)
        .with_ws(io.pins.gpio4)
        .with_dout(io.pins.gpio5)
        .build();
    
    
    //STEP = (200.0 * 80)/44100
    let mut phase: f32 = 0.0;
    let mut j = 0;
    for _ in 0..TX_BUFF_LEN/2{
        let bytes = sin_i16(j).to_ne_bytes();
        tx_buff[j] = bytes[0];
        tx_buff[j+1] = bytes[1];
        
        phase += STEP;
        j+= 2;
        if phase > SINE.len() as f32{
            phase = 0.0;
        }
    }

    let mut transfer = i2s_tx.write_dma_circular(&tx_buff).unwrap();
    
    let mut filler: [u8; TX_BUFF_LEN] = [0; TX_BUFF_LEN];
    loop {
        if transfer.available() > 1{
            let mut size = usize::min(transfer.available(), TX_BUFF_LEN);
            if size & 1 != 0 {size -= 1;}
            fill_transfer_buffer(&mut filler, size, &mut phase);
            transfer.push(&filler[..size]).unwrap();
        }
    }
}

fn fill_transfer_buffer(buffer: &mut[u8],size: usize, phase: &mut f32){
    let mut j = 0;
    while j < size{
        let bytes = sin_i16_bytes(*phase as usize); 
        buffer[j] = bytes.0;
        buffer[j+1] = bytes.1;
        j += 2;
        
        *phase += STEP;
        if *phase > SINE.len() as f32{
            *phase = 0.0;
        }
    }
}

