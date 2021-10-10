f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day3\\input.txt")
contents = f.read()

# contents = '^v'

visited = [[0, 0]]

x = 0
y = 0


for v in visited:
    if v == '^':
        y += 1
    if v == 'v':
        y -= 1
    if v == '>':
        x += 1
    if v == '<':
        x -= 1

    if not visited.__contains__([x, y]):
        visited.append([x, y])

print(len(visited))
