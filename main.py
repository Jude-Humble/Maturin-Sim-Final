import rocket_sim
import matplotlib
import math

matplotlib.use("Agg")

to_radians = 0.0174533

wet_mass = 1.2  # kg
dry_mass = 0.8  # kg
dm = 0.1  # kg/s
thrust = 25  # Newtons
time_step = 0.01  # 1 / step rate
duration = 2000  # cycles
cp = 0.65 # from top
cg = 0.45 #from top
cmp = 0.5 #idk what this is
drag_coefficient = 0.5 #self explanitory
width = 0.08 # x m
height = 1.0 # y m
depth = 0.08 # z m
mass = rocket_sim.MassStruct(dry_mass, wet_mass,)
dimensions = rocket_sim.Vec3f()
dimensions.redefine(width, height, depth)
rotational = rocket_sim.RotateStruct(cp, cg, cmp, drag_coefficient, dimensions, mass)

#starting thrust vector in degrees
tx = 10
ty = 0

in_thrust = rocket_sim.Vec3f()
in_thrust.redefine(
    math.sin(tx * to_radians), 
    math.cos(ty * to_radians), 
    0
)

import matplotlib.pyplot as plt

test = rocket_sim.Rocket(mass, dm, rotational, thrust, time_step, duration, in_thrust)
test.uncontrolled_sim()
# test.print_history()

pos_height = test.get_history(0, 1)
pos_x = test.get_history(0, 0)
vel_vert = test.get_history(1, 1)
acc_vert = test.get_history(2, 1)
pos_mass = test.get_history(4, 0)
thrust_x = test.get_history(5, 0)
thrust_y = test.get_history(5, 1)

plt.figure()
plt.plot(pos_height, label="height")
plt.plot(vel_vert, label="vertical velocity")
plt.plot(acc_vert, label="vertical acceleration")
plt.plot(pos_mass, label="mass")
plt.legend()
plt.savefig("sim_data.png")
plt.close()

plt.figure()
plt.plot(pos_x, pos_height)
plt.savefig("traj.png")
plt.close()

plt.figure()
plt.plot(thrust_x, "x")
plt.plot(thrust_y, "y")
plt.legend()
plt.savefig("thrust_data.png")
plt.close()
