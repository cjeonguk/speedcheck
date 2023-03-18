import random

l: list[int] = []

for i in range(20000):
        l.append(random.randrange(0, 25000))

print(l)