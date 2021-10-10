f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2020\\day6\\input.txt")
contents = f.readlines()

ans = 0
group = set()
contents.append('')
for line in contents:
    line = line.strip()
    if not line:
        ans += len(group)
        group = set()
    else:
        for qu in line:
            group.add(qu)
print(ans)
