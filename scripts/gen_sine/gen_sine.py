import math
import matplotlib.pyplot as plt


DEBUG = True
SAVE_DATA = True 
OPTIMIZED = False
CSV = False
OUTPUT_PATH = "./out/"

AMPLITUDE = 32_767
SAMPLES = 1024 
SPL = 8

def save_data(wave, file_path, spl = SPL):
    with open(file_path, 'wt') as file:
        string = "[\n";
        for i in  range(0,len(wave), spl):
            string += '\t';
            for j in range(0, spl):
                string += "{}, ".format(wave[i + j]);
            string += '\n';

        string += '];';
        
        file.write(string)

#==============================================================================================================#

def build_full_wave(base):
    half_wave_len = len(base) * 2;
    full_wave = base + ([0] * (3 * len(base)));

    for i in range(len(base)):
        full_wave[half_wave_len - i] = full_wave[i];
        full_wave[half_wave_len + i] = -full_wave[i];
        full_wave[len(full_wave) - max(i, 1)] = -full_wave[i];
    

    #Special cases
    full_wave[len(base)] = base[len(base) - 1];
    full_wave[half_wave_len + len(base)] = -base[len(base) - 1];

    return full_wave;

#==============================================================================================================#

sine = [0] * SAMPLES;

divisor = SAMPLES
if OPTIMIZED:
    divisor = SAMPLES * 4;


for i in range(SAMPLES):
    sine[i] = int(AMPLITUDE * math.sin(2* math.pi * i/divisor));

if DEBUG:
    full_sine = sine
    if OPTIMIZED:
        full_sine = build_full_wave(sine);

    plt.grid()
    plt.plot(full_sine)
    plt.show()

if SAVE_DATA:
    save_data(sine, OUTPUT_PATH + 'sine.txt');




            

