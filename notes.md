## Signal Multiply
* have frequency to multiply by, have smaples
  
1. 
   * samples in vram = [a,b,c,d]
   * generated_signal = [x,y,z,u]
   * I -> generated I
   * Q -> generated Q

2. 
3. pointwise multiply generated signal to samples
input should be "ElementParameter" of complex f32. Because complex, we must get I array and Q array out separately

sps: amount of data per seconds, samples per symbol
sps is the length of frequency bins array
each bin is an increment
frequency per bin is sample_rate / 2 * sps
each bin is an increment from 0 to sample_rate / 2
each bin is taking a step
if each size is sample rate / sps
if nyquist was 5 mhz, 

sample_rate / sps = distance between each bin
   because SPS is # of bins, and the total range of the frequencies is the sample_rate / 2 and the -sample_rate / 2
* make sure to use the right step!
* sps is total number of bins!
* so now the bandpass has 2 cutoff frequencies, because it is low and highpass filters combined
* thus we have 2 cutoffs
* we need cutoff for high pass and cutoff for low pasS

### Bandpass
* we have start frequency and end frequency.
* is frequency below the end, and above the start freuqency then multiply by 0
* one rolloff