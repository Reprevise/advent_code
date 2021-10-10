import itertools

f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2020\\day9\\input.txt")
contents = f.read().splitlines()

p1 = 0
p2 = 0

contents = list(map(int, contents))
contents.append('')
for i in range(25, len(contents)):
    ok = True
    prev = contents[i-25:i]
    for y, z in itertools.combinations(prev, 2):
        if y+z == contents[i]:
            ok = False
    if ok:
        print(contents[i])
        break
