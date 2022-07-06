# %%
import numpy as np
import soundfile as sf
import matplotlib as plt

# %% Load file
#data = sf.read("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/transcoded_CA01_01.wav")
data = sf.read("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav")

data = data[0]
#data = np.float16(data)
# %%
fft_size = 262144
out = np.fft.rfft(data, n=fft_size)
print(np.shape(out))
print(out[0])
print(out[1])
print(out[2])
print(out[3])
print(out[4])
print(out[5])
print(f"Mean: {np.mean(data)}")
print(f"sum:  {np.sum(data)}")


# %% Export faulty patch

import numpy as np
import soundfile as sf
import matplotlib as plt

ref_data = sf.read("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav")
deg_data = sf.read("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/transcoded_CA01_01.wav")
print(ref_data[1])
ref_data = ref_data[0]
deg_data = deg_data[0]

print(ref_data[71040])
print(ref_data[99839])
# %%
print(deg_data[71040 - 1] / -0.017098)
print(deg_data[71040] / -0.0141978)
print(deg_data[71040 + 1] / -0.011365)
print(deg_data[99839] / 6.82585e-05)

# %%
sf.write('ref_patch.wav', ref_data[71040:99839], 48000, 'PCM_16')


# %%

sf.write('deg_patch.wav', deg_data[71040:99839], 48000, 'PCM_16')
# %%
