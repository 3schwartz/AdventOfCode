import z3  # z3-solver

hailstones = []
for line in open('../../data/day24_data.txt'):
    hailstones.append([int(i) for i in line.replace('@', ',').split(',')])

position = z3.RealVector('p', 6)  # xp, yp, zp, xv, yv, zv
time = z3.RealVector('t', len(hailstones))  # hit one stone each time unit
solver = z3.Solver()
for i in range(len(hailstones)):
    x_constrain = position[0] + time[i] * position[3] == hailstones[i][0] + time[i] * hailstones[i][3]
    y_constrain = position[1] + time[i] * position[4] == hailstones[i][1] + time[i] * hailstones[i][4]
    z_constrain = position[2] + time[i] * position[5] == hailstones[i][2] + time[i] * hailstones[i][5]
    solver.add(x_constrain)
    solver.add(y_constrain)
    solver.add(z_constrain)
solver.check()
model = solver.model()
print(f"Part 2: {model.eval(sum(position[:3]))}")
