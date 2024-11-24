import numpy as np
import matplotlib.pyplot as plt


FREQUENCY = 60.0
SAMPLE_RATE = 44100
ASSUMED_SR = 44100

SAMPLES = 1024
AMPLITUDE = 32_767

FFT = True

def gen_sine(samples: int, amp: float):
    sine = np.zeros(samples, np.int16);
    for i in range(samples):
        sine[i] = amp * np.sin(2* np.pi * (i/samples))

    return sine;

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

signal = simulate_signal(SAMPLE_RATE, ASSUMED_SR, FREQUENCY, sine)

t = np.linspace(0, 1, SAMPLE_RATE)

plt.grid()
plt.plot(t, signal)
plt.show()

if FFT:
    signal_fft = np.fft.rfft(signal)

    plt.plot(signal_fft)
    plt.show()
