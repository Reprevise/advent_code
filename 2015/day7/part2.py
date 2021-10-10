import operator

f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day7\\input.txt")
contents = f.read().splitlines()

PARTS = {}
CACHE = {}


def compute(value):
    if value in CACHE:
        return CACHE[value]

    if value.isdigit():
        return int(value)

    value = PARTS[value]

    if 'NOT' in value:
        value_a = value.split(' ')[1]
        return ~ compute(value_a)

    try:
        (value_a, operation, value_b) = value.split(' ')

        computed_a = compute(value_a)
        CACHE[value_a] = computed_a

        computed_b = compute(value_b)
        CACHE[value_b] = computed_b

        if operation == 'AND':
            computed = compute(value_a) & compute(value_b)
        elif operation == 'OR':
            computed = compute(value_a) | compute(value_b)
        elif operation == 'LSHIFT':
            computed = compute(value_a) << compute(value_b)
        elif operation == 'RSHIFT':
            computed = compute(value_a) >> compute(value_b)
        else:
            print("Error")
            return

        return computed
    except ValueError:
        return compute(value)


for x in contents:
    (operation, name) = x.split(' -> ')
    PARTS[name] = operation

signal_a = compute('a')

CACHE = {}

PARTS['b'] = str(signal_a)

solution = compute('a')
print(solution)
