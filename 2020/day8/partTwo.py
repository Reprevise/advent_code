import sys

f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2020\\day8\\input.txt")
contents = f.read().splitlines()

p1 = 0
p2 = 0

for change in range(len(contents)):
    nc = list(contents)
    if nc[change].split()[0] == 'nop':
        nc[change] = 'jmp '+nc[change].split()[1]
    elif nc[change].split()[0] == 'jmp':
        nc[change] = 'nop '+nc[change].split()[1]
    else:
        continue
    t = 0
    ip = 0
    acc = 0
    while 0 <= ip < len(nc) and t < 1000:
        t += 1
        words = nc[ip].split()
        if words[0] == 'acc':
            acc += int(words[1])
            ip += 1
        elif words[0] == 'nop':
            ip += 1
        elif words[0] == 'jmp':
            ip += int(words[1])
    if ip == len(nc):
        print(acc)
