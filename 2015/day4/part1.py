f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day4\\input.txt")
contents = f.readlines()

vowels = ['a', 'e', 'i', 'o', 'u']
bad = ['ab', 'cd', 'pq', 'xy']

nice = []

for x in contents:
    vCount = 0
    isBad = False
    double = False

    for y in x:
        if vowels.__contains__(y):
            vCount += 1

    for y in bad:
        if x.__contains__(y):
            isBad = True

    i = 0
    if not isBad:
        while i < len(x)-1:
            if x[i] == x[i+1]:
                double = True
                break
            i += 1

    if vCount >= 3 and double and not isBad:
        nice.append(x)

print(len(nice))
