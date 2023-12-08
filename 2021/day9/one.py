with open('input.txt') as f:
    lines = f.read().splitlines(False)

    grid: list[list[int]] = []
    for line in lines:
        grid.append([int(char) for char in line])

    lows: list[int] = []

    def is_smaller(n: int, c: int, r: int):
        # check left
        left = True
        right = True
        top = True
        bottom = True
        if c - 1 >= 0:
            left = grid[r][c - 1] > n
        if c + 1 <= 99:
            right = grid[r][c + 1] > n
        if r - 1 >= 0:
            top = grid[r - 1][c] > n
        if r + 1 < len(grid):
            bottom = grid[r + 1][c] > n
        return left and right and top and bottom

    for row, numbers in enumerate(grid):
        for column, n in enumerate(numbers):
            if n == 9:
                continue
            if is_smaller(n, column, row):
                lows.append(n)

    print(sum(map(lambda x: x + 1, lows)))
