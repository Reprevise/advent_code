import math

f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day2\\input.txt")
contents = f.read().splitlines()

sqFt = 0

for i, v in enumerate(contents):
    numbers = list(map(int, v.split('x')))
    numbers.sort()
    bow = math.prod(numbers)
    ribbon = (numbers[0]*2) + (numbers[1]*2)
    sqFt += ribbon + bow

print(sqFt)
