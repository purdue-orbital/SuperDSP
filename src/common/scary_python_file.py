# keep this file in here to make the rust people angry
from scipy.fftpack import fftshift


x = [x for x in range(20)]

print(x)

y = fftshift(x)

print(list(y))