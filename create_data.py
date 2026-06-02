
import random
import os

random.seed(42)
data = ''.join(random.choice('01') for _ in range(500_000))
with open('large_binary.txt', 'w') as f:
    _ = f.write(data)
