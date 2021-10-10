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

longest = 0
for items in permutations(places):
    dist = sum(map(lambda x, y: distances[x][y], items[:-1], items[1:]))
    longest = max(longest, dist)

print("longest: %d" % (longest))
