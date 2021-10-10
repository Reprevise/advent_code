f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2020\\day4\\input.txt")
contents = f.readlines()

ans = 0
passport = {}
for line in contents:
    line = line.strip()
    if not line:
        valid = True
        if 'byr' in passport:
            byr = passport['byr']
            byr_n = int(byr)
            if len(byr) != 4 or not (1920 <= byr_n <= 2002):
                valid = False
        else:
            valid = False
        if 'iyr' not in passport or not (2010 <= int(passport['iyr']) <= 2020):
            valid = False
        if 'eyr' not in passport or not (2020 <= int(passport['eyr']) <= 2030):
            valid = False
        if 'hgt' in passport:
            ht = passport['hgt']
            try:
                ht_n = int(ht[:-2])
                if ht.endswith('in'):
                    if not 59 <= ht_n <= 76:
                        valid = False
                elif ht.endswith('cm'):
                    if not 150 <= ht_n <= 193:
                        valid = False
                else:
                    valid = False
            except:
                valid = False
        else:
            valid = False
        if 'hcl' in passport:
            hcl = passport['hcl']
            if hcl[0] != '#' or any([c not in '0123456789abcdef' for c in hcl[1:]]):
                valid = False
        else:
            valid = False
        if 'ecl' not in passport or passport['ecl'] not in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']:
            valid = False
        if 'pid' in passport:
            pid = passport['pid']
            if len(pid) != 9 or any([c not in '0123456789' for c in pid]):
                valid = False
        else:
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
