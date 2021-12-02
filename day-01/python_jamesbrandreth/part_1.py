
numbers = []
with open('./input.txt') as f:
    for line in f:
        numbers.append(int(line))

increases = 0
for i in range(1, len(numbers)):
    if numbers[i] > numbers[i-1]:
        increases += 1

print(increases)
