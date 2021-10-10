def get_slack(length, width, height):
    return min([length*width, width*height, height*length])


f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day2\\input.txt")
contents = f.readlines()

sqFt = 0

for v in contents:
    numbers = list(map(int, v.split('x')))
    length = numbers[0]
    width = numbers[1]
    height = numbers[2]
    slack = get_slack(length, width, height)
    lw = 2*length*width
    wh = 2*width*height
    hl = 2*height*length
    sqFt += lw + wh + hl + slack

print(sqFt)
