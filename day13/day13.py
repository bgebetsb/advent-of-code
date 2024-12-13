from sympy import symbols, Eq, solve, Integer

def calculate(ax, ay, bx, by, px, py):
    button_a, button_b = symbols('button_a button_b', integer=True)
    
    eq_x = Eq(button_a * ax + button_b * bx, px)
    eq_y = Eq(button_a * ay + button_b * by, py)
    
    solution = solve((eq_x, eq_y), (button_a, button_b))
    if solution:
        return solution[button_a] * 3 + solution[button_b]
    else:
        return 0

def split_line(line):
    parts = line.split(',')
    numbers = []
    for part in parts:
        part = part.replace('+', '=')
        number = part.split('=')
        numbers.append(int(number[1]))
    return tuple(numbers)

def read_file(filename):
    part1 = 0
    part2 = 0
    with open(filename, 'r') as file:
        while True:
            lines = [file.readline().strip() for _ in range(3)]
            if not any(lines):
                break
            ax, ay = split_line(lines[0])
            bx, by = split_line(lines[1])
            px, py = split_line(lines[2])
            part1 += calculate(ax, ay, bx, by, px, py)
            px += 10000000000000
            py += 10000000000000
            part2 += calculate(ax, ay, bx, by, px, py)
            file.readline() # Skip empty line afterwards
    print("Part 1: " + str(part1))
    print("Part 2: " + str(part2))


if __name__ == '__main__':
    read_file('input.txt')
