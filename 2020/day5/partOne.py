f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2020\\day5\\input.txt")
contents = f.readlines()

p1 = 0

for line in contents:
    line = line.strip()

    row = 0
    rp = 64

    col = 0
    cp = 4

    for c in line:
        if c == 'B':
            row += rp
        rp /= 2

        if c == 'R':
            col += cp
            cp /= 2
        elif c == 'L':
            cp /= 2
    seat_id = row*8+col
    p1 = max(p1, seat_id)

print(p1)
