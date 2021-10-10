import sys

f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2020\\day8\\input.txt")
contents = f.read().splitlines()

p1 = 0
p2 = 0

seen = set()
ip = 0
acc = 0
while True:
    if ip in seen:
        print(acc)
        sys.exit(0)
    seen.add(ip)
    words = contents[ip].split()
    if words[0] == 'acc':
        acc += int(words[1])
        ip += 1
    elif words[0] == 'nop':
        ip += 1
    elif words[0] == 'jmp':
        ip += int(words[1])
