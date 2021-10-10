import sys
from itertools import permutations

places = set()
distances = dict()
for line in open('C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day9\\input.txt'):
    (source, _, dest, _, distance) = line.split()
    places.add(source)
    places.add(dest)
    distances.setdefault(source, dict())[dest] = int(distance)
    distances.setdefault(dest, dict())[source] = int(distance)

shortest = sys.maxsize
for items in permutations(places):
    dist = sum(map(lambda x, y: distances[x][y], items[:-1], items[1:]))
    shortest = min(shortest, dist)

print("shortest: %d" % (shortest))
