import itertools

f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2020\\day10\\input.txt")
contents = f.read().splitlines()

p1 = 0
p2 = 0

contents = list(map(int, contents))
contents.append(0)
contents = sorted(contents)
contents.append(max(contents)+3)

n1 = 0
n3 = 0

for i in range(len(contents)-1):
    d = contents[i+1]-contents[i]
    if d == 1:
        n1 += 1
    elif d == 3:
        n3 += 1
print(n1*n3)
