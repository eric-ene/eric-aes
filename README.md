# Eric's AES Library
This implements AES-128, as well as its key schedule.

If you're here because youre marking this, the main process of the AES implementation is in [aestools.rs](src/aestools.rs), with many of the functions from AES implemented in [misctools.rs](src/misctools.rs). The matrix multiplication in $GF(2^8)$ is implemented in [matrix.rs](src/matrix.rs)