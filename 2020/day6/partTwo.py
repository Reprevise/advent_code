import string

f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2020\\day6\\input.txt")
contents = f.readlines()

ans = 0
contents.append('')
qs = set(string.ascii_lowercase)
for line in contents:
    line = line.strip()
    if not line:
        ans += len(qs)
        qs = set(string.ascii_lowercase)
    else:
        for ch in string.ascii_lowercase:
            if ch not in line and ch in qs:
                qs.remove(ch)
print(ans)
