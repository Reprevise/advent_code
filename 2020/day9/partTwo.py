from itertools import combinations

f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2020\\day9\\input.txt")
contents = f.read().splitlines()
contents = list(map(int, contents))

pre = 25

p1 = None
for i in range(pre, len(contents)):
    prev = contents[i-pre:i]
    if p1 is None and (not any([x+y == contents[i] for x, y in combinations(prev, 2)])):
        p1 = contents[i]

p2 = None
for i in range(len(contents)):
    for j in range(i+2, len(contents)):
        ys = contents[i:j]
        if sum(ys) == p1:
            p2 = min(ys)+max(ys)
print(p1)
print(p2)
