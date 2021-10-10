import re

f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2020\\day4\\input.txt")
contents = f.readlines()

ans = 0
passport = {}
for line in contents:
    line = line.strip()
    if not line:
        valid = True
        for field in ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']:
            if field not in passport:
                valid = False
        if valid:
            ans += 1
        passport = {}
    else:
        words = line.split()
        for word in words:
            k, v = word.split(':')
            passport[k] = v

print(ans)
