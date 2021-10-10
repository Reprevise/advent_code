from json import loads

file = open('C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day12\\input.txt')


def n(j):
    if type(j) == int:
        return j
    if type(j) == list:
        return sum([n(j) for j in j])
    if type(j) != dict:
        return 0
    if 'red' in j.values():
        return 0
    return n(list(j.values()))


print(n(loads(file.read())))
