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

def gen_envelope(sample_rate: int):
    attack_rate = 4.0/sample_rate
    decay_rate = -2.0/sample_rate
    sustain_level = 0.5
    release_rate = -4.0/sample_rate
    value = 0.0
    env = np.zeros(sample_rate)
    state = 0 # attack
    step = attack_rate
    for i in range(sample_rate):
        env[i] = value
        value += step
        if state == 0 and value > 1.0:
            value = 1.0
            state = 1 # decay
            step = decay_rate
        if state == 1 and value < sustain_level:
            value = sustain_level
            state = 2 # sustain
            step = 0.0
        if state == 2 and i/sample_rate > 0.75: # detrigger
            state = 3 # release
            step = release_rate
        if state == 3 and value < 0.0:
            value = 0.0
            state = 4 # off
            step = 0.0
    return env

sine = gen_sine(SAMPLES, AMPLITUDE);
saw_tooth = gen_saw_tooth(SAMPLES, AMPLITUDE);
square_wave = gen_square_wave(SAMPLES, AMPLITUDE);
triangle_wave = gen_triangle_wave(SAMPLES, AMPLITUDE);
envelope = gen_envelope(SAMPLE_RATE)

signal = simulate_signal(SAMPLE_RATE, ASSUMED_SR, FREQUENCY, triangle_wave);
signal = [samp*env for samp, env in zip(signal, envelope)]

t = np.linspace(0, 1, SAMPLE_RATE)

plt.grid()
plt.plot(t, signal)
plt.show()

if FFT:
    signal_fft = np.fft.rfft(signal)

    plt.plot(signal_fft)
    plt.show()
