# inputs:list = list(map(float, input().split(",")))
inputs: list[float] = []

for i in range(15):
    inputs.append(float(input()))

sum = 0
for i in inputs:
    sum += i

print(sum)
print(sum/len(inputs))