f = open("C:\\Users\\thesl\\Desktop\\advent_code\\2015\\day6\\input.txt")
contents = f.read().splitlines()

lights = [[0 for inner in range(1000)] for outer in range(1000)]

for x in contents:
    x = x.split(' ')
    if x[0] == 'toggle':
        (start, stop) = (x[1], x[3])

        (start_x, start_y) = start.split(',')
        (stop_x, stop_y) = stop.split(',')

        for light_x in range(int(start_x), int(stop_x)+1):
            for light_y in range(int(start_y), int(stop_y)+1):
                lights[light_x][light_y] += 2
    elif x[1] == 'off':
        (start, stop) = (x[2], x[4])
        (start_x, start_y) = start.split(',')
        (stop_x, stop_y) = stop.split(',')

        for light_x in range(int(start_x), int(stop_x)+1):
            for light_y in range(int(start_y), int(stop_y)+1):
                lights[light_x][light_y] -= 1
                if lights[light_x][light_y] < 0:
                    lights[light_x][light_y] = 0
    elif x[1] == 'on':
        (start, stop) = (x[2], x[4])
        (start_x, start_y) = start.split(',')
        (stop_x, stop_y) = stop.split(',')

        for light_x in range(int(start_x), int(stop_x)+1):
            for light_y in range(int(start_y), int(stop_y)+1):
                lights[light_x][light_y] += 1

on_lights = 0
for row in lights:
    on_lights += sum(row)

print(on_lights)
