import numpy as np
import matplotlib.pyplot as plt


FREQUENCY = 60.0
SAMPLE_RATE = 44100
ASSUMED_SR = 44100

SAMPLES = 1024
AMPLITUDE = 32_767

FFT = False 

#===================================================== WAVE FUNCTTIONS =====================================================#

def gen_sine(samples: int, amp: float):
    sine = np.zeros(samples, np.int16);
    for i in range(samples):
        sine[i] = amp * np.sin(2* np.pi * (i/samples))

    return sine;

def gen_saw_tooth(samples: int, amp: float):
    saw_tooth = np.zeros(samples, np.int16);
    for i in range(samples):
        saw_tooth[i] = i*amp/samples;
    return saw_tooth;

def gen_square_wave(samples: int, amp: float):
    square_wave = np.zeros(samples, np.int16);
    for i in range(int(samples/2)):
        square_wave[i] = amp;

    return square_wave;

def gen_triangle_wave(samples: int, amp: float):
    triangle_wave = np.zeros(samples, np.int16);
    half = int(samples/2);

    for i in range(half):
        triangle_wave[i] = 2*i*amp/samples;

    for (counter, i) in enumerate(range(half, samples)):
        triangle_wave[i] = amp - counter * 2 *amp/samples;
    
    return triangle_wave;

#=================================================== WAVE FUNCTTIONS - END ===================================================#

def simulate_signal(sample_rate: int, assumed_sr: int,freq: float, table):
    step = (freq * len(table))/assumed_sr
    phase = 0.0
    sim_sine = np.zeros(sample_rate)
    for i in range(sample_rate):
        sim_sine[i] = table[int(phase)]
        phase += step 
        
        if phase > len(table):
            phase = 0.0
    
    return sim_sine

sine = gen_sine(SAMPLES, AMPLITUDE);
saw_tooth = gen_saw_tooth(SAMPLES, AMPLITUDE);
square_wave = gen_square_wave(SAMPLES, AMPLITUDE);
triangle_wave = gen_triangle_wave(SAMPLES, AMPLITUDE);

signal = simulate_signal(SAMPLE_RATE, ASSUMED_SR, FREQUENCY, triangle_wave);

t = np.linspace(0, 1, SAMPLE_RATE)

plt.grid()
plt.plot(t, signal)
plt.show()

if FFT:
    signal_fft = np.fft.rfft(signal)

    plt.plot(signal_fft)
    plt.show()
