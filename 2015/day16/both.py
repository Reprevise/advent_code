file = open('C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day16\\input.txt')
lines = file.read().splitlines()

a_n = 0

for line in lines:
    a_n += 1
    aunt = {}
    o = line.split(' ', 2)
    p = o[2].split(', ')
    for v in p:
        k, val = v.split(': ')
        aunt[k] = int(val)
    if 'akitas' in aunt:
        continue
    if 'vizslas' in aunt:
        continue
    if 'children' in aunt:
        ch = aunt['children']
        if ch != 3:
            continue
    else:
        continue
    if 'cats' in aunt:
        cats = aunt['cats']
        if cats != 7:
            continue
    else:
        continue
    if 'samoyeds' in aunt:
        samoyeds = aunt['samoyeds']
        if samoyeds != 2:
            continue
    else:
        continue
    if 'goldfish' in aunt:
        goldfish = aunt['goldfish']
        if goldfish != 5:
            continue
    else:
        continue
    if 'trees' in aunt:
        trees = aunt['trees']
        if trees != 3:
            continue
    else:
        continue
    if 'cars' in aunt:
        cars = aunt['cars']
        if cars != 2:
            continue
    else:
        continue
    if 'perfumes' in aunt:
        perfumes = aunt['perfumes']
        if perfumes != 3:
            continue
    else:
        continue
    print(a_n)
    break
