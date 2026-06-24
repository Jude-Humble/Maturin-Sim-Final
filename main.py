import rocket_sim
import matplotlib

matplotlib.use("Agg")

wet_mass = 5.5  # kg
dry_mass = 1  # kg
dm = 2  # kg/s
thrust = 100  # Newtons
time_step = 0.05  # 1 / step rate
duration = 2000  # cycles
cp = # from top
cg = #from top
cmp = #idk what this is
drag_coefficient = #self explanitory
width = # x m
height = # y m
depth = # z m

mass = test_rocket.MassStruct.new(dry_mass, wet_mass,)
rotational = test_rocket.RotateStruct.new(cp, cg, cmp, drag_coefficient)
dimensions = rocket_sim.Vec3f.new()

import matplotlib.pyplot as plt

test = rocket_sim.Rocket(mass, dm, rotational, thrust, time_step, duration)
test.uncontrolled_sim()
# test.print_history()

pos_height = test.get_history(0, 1)
vel_vert = test.get_history(1, 1)
acc_vert = test.get_history(2, 1)
pos_mass = test.get_history(4, 0)

plt.plot(pos_height, label="height")
plt.plot(vel_vert, label="vertical velocity")
plt.plot(acc_vert, label="vertical acceleration")
plt.plot(pos_mass, label="mass")

plt.legend()

plt.savefig("graph.png")
