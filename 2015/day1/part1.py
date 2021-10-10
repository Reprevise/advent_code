f = open("input.txt")
contents = f.read()

floor = 0

for i, v in enumerate(contents):
    if v == '(':
        floor = floor + 1
    elif v == ')':
        floor = floor - 1


print(floor)
