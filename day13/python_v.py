from sympy import symbols, Eq, solve, Integer

p1, p2 = symbols('p1 p2', integer=True)

a, b = 94, 34  # Example: Button 1 moves 3 units down and 4 units right
c, d = 22, 67  # Example: Button 2 moves 5 units down and 6 units right

# Define the target coordinates
x_target = 84000  # Target x-coordinate
y_target = 54000  # Target y-coordinate

# Define the equations for x and y targets
eq_x = Eq(p1 * a + p2 * c, x_target)
eq_y = Eq(p1 * b + p2 * d, y_target)

# Try to maximize p2 by solving for p1 in terms of p2 and checking if p1 is an integer
# solutions = []
solutions = solve((eq_x, eq_y), (p1, p2))
# for p2_value in range(x_target // d, -1, -1):  # Try p2 from max to 0
#     print(p2_value)
#     eq_p1 = eq_x.subs(p2, p2_value)  # Substitute p2_value into the x equation
#     p1_solution = solve(eq_p1, p1)
#     
#     if p1_solution and p1_solution[0] >= 0:  # Check if p1 is valid (non-negative integer)
#         p1_value = p1_solution[0]
#         eq_p2 = eq_y.subs(p1, p1_value)  # Check y equation for the same p1
#         if solve(eq_p2, p2) == [p2_value]:  # Validate if this p2 works
#             solutions.append((p1_value, p2_value))

# If there are multiple solutions, pick the one with the highest p2
if solutions:
    # If solutions is a list, iterate through all solutions
    if isinstance(solutions, list):
        best_solution = max(solutions, key=lambda sol: sol[p2])
    elif isinstance(solutions, dict):  # If there's only one solution
        best_solution = solutions
    else:
        best_solution = None

    # Extract p1 and p2 from the best solution
    if best_solution:
        b1 = best_solution[p1]
        highest_b2 = best_solution[p2]
        print(f"Button A: {b1}, Button B: {highest_b2}")
else:
    print(0)
