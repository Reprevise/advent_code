i = [char for char in '1321131112']
# i = [char for char in '111221']

current = i

co = 1
s = ''
for _ in range(40):
    print(_)
    for x in range(len(current)-1):
        c = current[x]
        n = current[x+1]
        if n == c:
            co += 1
        else:
            s += str(co) + c
            co = 1
        if x == len(i)-2:
            s += str(co) + n
            current = [char for char in s]
            print(len(current))


print(len(current))
