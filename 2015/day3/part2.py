f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day3\\input.txt")
contents = f.read()

# contents = '^v'

visited = [[0, 0]]

a = 0
b = 0

x = 0
y = 0

roboSanta = False

for i, v in enumerate(contents):
    if not roboSanta:
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

    else:
        if v == '^':
            b += 1
        if v == 'v':
            b -= 1
        if v == '>':
            a += 1
        if v == '<':
            a -= 1

        if not visited.__contains__([a, b]):
            visited.append([a, b])

    roboSanta = not roboSanta

    # print(x, y)
    # print(visited)


print(len(visited))
