with open('C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day8\\input.txt') as f:
    print(sum(len(_) - 1 - len(eval(_)) for _ in f))
