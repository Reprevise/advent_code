f = open("part1input.txt")
contents = f.read()

floor = 0

for i, v in enumerate(contents):
    if floor == -1:
        print("Santa enters basement at " + str(i))
        break
    if v == '(':
        floor = floor + 1
    elif v == ')':
        floor = floor - 1
