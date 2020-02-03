#!/usr/bin/env python3

# Generate an LED gamma-correction table

gamma = 2.8 # Correction factor
max_in = 255
max_out = 255

print("// Format this before using")
print("const GAMMA8: [u8; 256] = [")
for i in range(max_in + 1):
    print(str(int(pow(i/max_in, gamma) * max_out + 0.5)) + ", ")
print("];")
