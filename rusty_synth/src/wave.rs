
pub mod constants{

pub const SINE: [i16; 512] = 
    [
        0, 402, 804, 1206, 1607, 2009, 2410, 2811, 
        3211, 3611, 4011, 4409, 4807, 5205, 5601, 5997, 
        6392, 6786, 7179, 7571, 7961, 8351, 8739, 9126, 
        9511, 9895, 10278, 10659, 11038, 11416, 11792, 12166, 
        12539, 12909, 13278, 13645, 14009, 14372, 14732, 15090, 
        15446, 15799, 16150, 16499, 16845, 17189, 17530, 17868, 
        18204, 18537, 18867, 19194, 19519, 19840, 20159, 20474, 
        20787, 21096, 21402, 21705, 22004, 22301, 22594, 22883, 
        23169, 23452, 23731, 24006, 24278, 24546, 24811, 25072, 
        25329, 25582, 25831, 26077, 26318, 26556, 26789, 27019, 
        27244, 27466, 27683, 27896, 28105, 28309, 28510, 28706, 
        28897, 29085, 29268, 29446, 29621, 29790, 29955, 30116, 
        30272, 30424, 30571, 30713, 30851, 30984, 31113, 31236, 
        31356, 31470, 31580, 31684, 31785, 31880, 31970, 32056, 
        32137, 32213, 32284, 32350, 32412, 32468, 32520, 32567, 
        32609, 32646, 32678, 32705, 32727, 32744, 32757, 32764, 
        32767, 32764, 32757, 32744, 32727, 32705, 32678, 32646, 
        32609, 32567, 32520, 32468, 32412, 32350, 32284, 32213, 
        32137, 32056, 31970, 31880, 31785, 31684, 31580, 31470, 
        31356, 31236, 31113, 30984, 30851, 30713, 30571, 30424, 
        30272, 30116, 29955, 29790, 29621, 29446, 29268, 29085, 
        28897, 28706, 28510, 28309, 28105, 27896, 27683, 27466, 
        27244, 27019, 26789, 26556, 26318, 26077, 25831, 25582, 
        25329, 25072, 24811, 24546, 24278, 24006, 23731, 23452, 
        23169, 22883, 22594, 22301, 22004, 21705, 21402, 21096, 
        20787, 20474, 20159, 19840, 19519, 19194, 18867, 18537, 
        18204, 17868, 17530, 17189, 16845, 16499, 16150, 15799, 
        15446, 15090, 14732, 14372, 14009, 13645, 13278, 12909, 
        12539, 12166, 11792, 11416, 11038, 10659, 10278, 9895, 
        9511, 9126, 8739, 8351, 7961, 7571, 7179, 6786, 
        6392, 5997, 5601, 5205, 4807, 4409, 4011, 3611, 
        3211, 2811, 2410, 2009, 1607, 1206, 804, 402, 
        0, -402, -804, -1206, -1607, -2009, -2410, -2811, 
        -3211, -3611, -4011, -4409, -4807, -5205, -5601, -5997, 
        -6392, -6786, -7179, -7571, -7961, -8351, -8739, -9126, 
        -9511, -9895, -10278, -10659, -11038, -11416, -11792, -12166, 
        -12539, -12909, -13278, -13645, -14009, -14372, -14732, -15090, 
        -15446, -15799, -16150, -16499, -16845, -17189, -17530, -17868, 
        -18204, -18537, -18867, -19194, -19519, -19840, -20159, -20474, 
        -20787, -21096, -21402, -21705, -22004, -22301, -22594, -22883, 
        -23169, -23452, -23731, -24006, -24278, -24546, -24811, -25072, 
        -25329, -25582, -25831, -26077, -26318, -26556, -26789, -27019, 
        -27244, -27466, -27683, -27896, -28105, -28309, -28510, -28706, 
        -28897, -29085, -29268, -29446, -29621, -29790, -29955, -30116, 
        -30272, -30424, -30571, -30713, -30851, -30984, -31113, -31236, 
        -31356, -31470, -31580, -31684, -31785, -31880, -31970, -32056, 
        -32137, -32213, -32284, -32350, -32412, -32468, -32520, -32567, 
        -32609, -32646, -32678, -32705, -32727, -32744, -32757, -32764, 
        -32767, -32764, -32757, -32744, -32727, -32705, -32678, -32646, 
        -32609, -32567, -32520, -32468, -32412, -32350, -32284, -32213, 
        -32137, -32056, -31970, -31880, -31785, -31684, -31580, -31470, 
        -31356, -31236, -31113, -30984, -30851, -30713, -30571, -30424, 
        -30272, -30116, -29955, -29790, -29621, -29446, -29268, -29085, 
        -28897, -28706, -28510, -28309, -28105, -27896, -27683, -27466, 
        -27244, -27019, -26789, -26556, -26318, -26077, -25831, -25582, 
        -25329, -25072, -24811, -24546, -24278, -24006, -23731, -23452, 
        -23169, -22883, -22594, -22301, -22004, -21705, -21402, -21096, 
        -20787, -20474, -20159, -19840, -19519, -19194, -18867, -18537, 
        -18204, -17868, -17530, -17189, -16845, -16499, -16150, -15799, 
        -15446, -15090, -14732, -14372, -14009, -13645, -13278, -12909, 
        -12539, -12166, -11792, -11416, -11038, -10659, -10278, -9895, 
        -9511, -9126, -8739, -8351, -7961, -7571, -7179, -6786, 
        -6392, -5997, -5601, -5205, -4807, -4409, -4011, -3611, 
        -3211, -2811, -2410, -2009, -1607, -1206, -804, -402, 
    ];

    pub const SINE_QUARTER: [i16 ; 128] =
    [
        0, 402, 804, 1206, 1607, 2009, 2410, 2811, 
        3211, 3611, 4011, 4409, 4807, 5205, 5601, 5997, 
        6392, 6786, 7179, 7571, 7961, 8351, 8739, 9126, 
        9511, 9895, 10278, 10659, 11038, 11416, 11792, 12166, 
        12539, 12909, 13278, 13645, 14009, 14372, 14732, 15090, 
        15446, 15799, 16150, 16499, 16845, 17189, 17530, 17868, 
        18204, 18537, 18867, 19194, 19519, 19840, 20159, 20474, 
        20787, 21096, 21402, 21705, 22004, 22301, 22594, 22883, 
        23169, 23452, 23731, 24006, 24278, 24546, 24811, 25072, 
        25329, 25582, 25831, 26077, 26318, 26556, 26789, 27019, 
        27244, 27466, 27683, 27896, 28105, 28309, 28510, 28706, 
        28897, 29085, 29268, 29446, 29621, 29790, 29955, 30116, 
        30272, 30424, 30571, 30713, 30851, 30984, 31113, 31236, 
        31356, 31470, 31580, 31684, 31785, 31880, 31970, 32056, 
        32137, 32213, 32284, 32350, 32412, 32468, 32520, 32567, 
        32609, 32646, 32678, 32705, 32727, 32744, 32757, 32764, 
    ];
    pub const SINE_QUARTER_BYTES: &[u8] = unsafe{
        core::slice::from_raw_parts(&SINE_QUARTER as *const _ as *const u8, 2 *SINE_QUARTER.len())
    };

    pub const SINE_BYTES: &[u8] = unsafe{
        core::slice::from_raw_parts(&SINE as *const _ as *const u8, 2*SINE.len())
    };


    pub const QUARTER_WAVE: usize = SINE_QUARTER.len();
    pub const HALF_WAVE: usize = SINE_QUARTER.len() * 2;
    pub const THREE_QUARTER_WAVE: usize = HALF_WAVE + QUARTER_WAVE;
    pub const FULL_WAVE: usize = SINE_QUARTER.len() * 4;

}


use self::constants::*;

pub fn sin_i16(x: usize) -> i16{
    let norm_x = x % FULL_WAVE;

    //Treat special cases
    match norm_x{
        QUARTER_WAVE => return SINE_QUARTER[norm_x - 1],
        HALF_WAVE => return SINE_QUARTER[0],
        THREE_QUARTER_WAVE => return -SINE_QUARTER[QUARTER_WAVE - 1],
        _ => (),
    };

    //First quadrant
    if norm_x < QUARTER_WAVE{
        return SINE_QUARTER[norm_x];
    }
    //Second quadrant
    if norm_x < HALF_WAVE{
        return SINE_QUARTER[HALF_WAVE - norm_x];
    }
    //Third quadrant
    if norm_x < THREE_QUARTER_WAVE{
        return -SINE_QUARTER[norm_x - HALF_WAVE];
    }

    //Fourth quadrant
    return -SINE_QUARTER[FULL_WAVE - norm_x];
}

pub fn sin_i16_bytes(x: usize) -> (u8, u8){
    let norm_x = (x % SINE.len()) << 1;
    return (SINE_BYTES[norm_x], SINE_BYTES[norm_x + 1]);
}

// pub fn sin_i16_bytes(x: usize) -> (u8, u8){
//    let norm_x = x % FULL_WAVE;
//     //Treat special cases
//     match norm_x{
//         QUARTER_WAVE => {
//             let i = (norm_x << 1) - 2;
//             return (SINE_QUARTER_BYTES[i], SINE_QUARTER_BYTES[i + 1]);
//         },
//         HALF_WAVE => return (SINE_QUARTER_BYTES[0], SINE_QUARTER_BYTES[1]),
//
//         THREE_QUARTER_WAVE => {
//             let i = (QUARTER_WAVE - 1) << 1;
//             return -SINE_QUARTER[QUARTER_WAVE - 1]
//         },
//         _ => (),
//     };
//
//     //First quadrant
//     if norm_x < QUARTER_WAVE{
//         return SINE_QUARTER[norm_x];
//     }
//     //Second quadrant
//     if norm_x < HALF_WAVE{
//         return SINE_QUARTER[HALF_WAVE - norm_x];
//     }
//     //Third quadrant
//     if norm_x < THREE_QUARTER_WAVE{
//         return -SINE_QUARTER[norm_x - HALF_WAVE];
//     }
//     
//     //Fourth quadrant
//     return -SINE_QUARTER[FULL_WAVE - norm_x];
// }

// pub fn sin_fast(mut x: f32) -> f32 {
//     // Normalize x to [-π, π]
//     while x > PI {
//         x -= TWO_PI;
//     }
//     while x < -PI {
//         x += TWO_PI;
//     }
//
//     // Use symmetry to map to [-π/2, π/2]
//     let sign = if x < 0.0 { -1.0 } else { 1.0 };
//     if x > PI_OVER_2 {
//         x = PI - x;
//     } else if x < -PI_OVER_2 {
//         x = -PI - x;
//     }
//
//     // Simplified polynomial approximation: sin(x) ≈ x * (1 - x^2 / 6)
//     let x2 = x * x;
//     x * (1.0 - x2 * (1.0 / 6.0)) * sign
// }
//
