import re
import json

file = open('C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day12\\input.txt')
contents = file.read()

numbers = re.findall(r'-?\d+', contents)
numbers = list(map(int, numbers))
print(sum(numbers))
