import re
from functools import reduce

cookies = []
for line in open('C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day15\\input.txt').readlines():
    g = re.match(
        r'(.*): capacity (\-?)(\d+), durability (\-?)(\d+), flavor (\-?)(\d+), texture (\-?)(\d+), calories (\-?)(\d+)', line).groups()
    cookie = [int(g[i]) * (-1 if g[i-1] else 1) for i in range(2, 12, 2)]
    cookies.append(cookie)

best_total = 0
best_for_calories = 0
for i in range(0, 101):
    for j in range(0, 101-i):
        for k in range(0, 101-i-j):
            l = 100 - i - j - k
            nums = [i*cookies[0][p] + j*cookies[1][p] + k*cookies[2]
                    [p] + l*cookies[3][p] for p in range(0, 5)]
            if min(nums) <= 0:
                continue
            total = reduce(lambda x, y: x * y, nums[:-1])
            best_total = max(total, best_total)
            if nums[4] == 500:
                best_for_calories = max(total, best_for_calories)

print(best_total, best_for_calories)
