f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day4\\input.txt")
contents = f.readlines()

nice = []

for x in contents:
    vCount = 0
    isBad = False
    double = False

    # i = 0
    # while i < len(x)-2:
    #     if x[i] == x[i+2]:
    #         bad = True
    #         break
    #     i += 1

    for i in range(1, len(x)-1):
        if(x[i - 1] == x[i + 1]):
            double = True
            break

    for j in range(0, len(x)-1):
        two = x[j] + x[j + 1]
        for i in range(j+2, len(x) - 1):
            if(two == (x[i] + x[i+1])):
                isBad = True

    # i = 0
    # if isBad:
    #     doubles = []
    #     while i < len(x)-1:
    #         if doubles.__contains__([x[i], x[i+1]]):
    #             double = True
    #             break
    #         else:
    #             doubles.append([x[i], x[i+1]])
    #         i += 1

    if isBad and double:
        nice.append(x)

print(len(nice))
