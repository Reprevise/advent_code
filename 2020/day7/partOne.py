from collections import deque

f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2020\\day7\\input.txt")
contents = f.readlines()

p1 = 0
p2 = 0

target = 'shinygoldbag'

E = {}

contents.append('')
for line in contents:
    line = line.strip()
    if line:
        words = line.split()
        container = words[0]+words[1]+words[2]
        container = container[:-1]
        if words[-3] == 'no':
            continue
        else:
            idx = 4
            while idx < len(words):
                bag = words[idx]+words[idx+1]+words[idx+2]+words[idx+3]
                if bag.endswith('.'):
                    bag = bag[:-1]
                if bag.endswith(','):
                    bag = bag[:-1]
                if bag.endswith('s'):
                    bag = bag[:-1]
                while any([bag.startswith(d) for d in '0123456789']):
                    bag = bag[1:]
                if bag not in E:
                    E[bag] = []
                E[bag].append(container)
                idx += 4
s = set()
Q = deque([target])
while Q:
    x = Q.popleft()
    if x in s:
        continue
    s.add(x)
    for y in E.get(x, []):
        Q.append(y)

print(len(s)-1)
