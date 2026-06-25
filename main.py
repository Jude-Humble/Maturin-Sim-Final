import rocket_sim
import matplotlib
import math

matplotlib.use("Agg")

to_radians = 0.0174533

wet_mass = 1.2  # mass of the rocket w/ all its fuel 
dry_mass = 0.8  # mass of the rocket w/o all its fuel 
dm = 0.1  # change in mass of the rocket due to fuel loss in kg/s 
thrust = 25  # thrust of the rocket motor in Newtons 
time_step = 0.01  #  time step of the simulation as 1 / cycles per second
duration = 2000  # maximum duration of the simulation in cycles. Simulation will end early if rocket goes below altitude of 0 meters 
cp = 0.65 # center of pressure for drag from the top of rocket in meters. Drag has yet to be integrated so don't worry abt this. You can leave it how it is 
cg = 0.45 # center of gravity of the rocket from the top of the rocket in meters. This is dynamic rn but I hope to make it dynamic in the future
cmp = 0.5 # center of pressure imposed by the rocket engine measured from the top in meters. Used in the calculation of the moment on the rocket body during flight
drag_coefficient = 0.5 # drag coefficient of the rocket in general. Drag is currently not being utilized so this value also doesn't matter that much. Just make sure to put something in so the code doesn't freak out
width = 0.08 # width of the rocket in meters. Basically the diameter 
height = 1.0 # height of the rocket in meters 
depth = 0.08 # depth of the rocket in meters. Also basically the diameter 

mass = rocket_sim.MassStruct(dry_mass, wet_mass,) # initialization of the mass struct for the rocket
dimensions = rocket_sim.Vec3f() # initialilzation of the rocket dimensions vector used for organization. I had to define it and then change it later because pyo3 was being fussy with having multiple constructors
dimensions.redefine(width, height, depth)
rotational = rocket_sim.RotateStruct(cp, cg, cmp, drag_coefficient, dimensions, mass)

#starting thrust vector in degrees
tx = 45
ty = 0

in_thrust = rocket_sim.Vec3f()
in_thrust.redefine(
    math.sin(tx * to_radians), # I messed up something when first writing the phsyics behidn the thrust vector and just changed the definition as a quick fix
    math.cos(ty * to_radians), 
    0
)

import matplotlib.pyplot as plt

rocket = rocket_sim.Rocket(mass, dm, rotational, thrust, time_step, duration, in_thrust)
rocket.uncontrolled_sim()
# test.print_history()

# get history method syntax is Rocket::get_history(tip: i8, id: i8) -> PyResult<Vec<f32>>
# basically, tip is the type of data and id is the value for which you want to fetch from the struct identified by tip. 
# if the type specified by tip is not a struct, than id could be quite anything really, it would still only return the single value
# The tips are as follows (Everything listed is under the struct RocketState)
# 0 => pos
# 1 => vel
# 2 => acc
# 3 => ang (converted to degrees for readability)
# 4 => mass
# 5 => thrust_vec (unit vector for thrust)
# and the ids can be assumed for all other than mass to be 0: x, 1: y, 2: z
pos_height = rocket.get_history(0, 1)
pos_x = rocket.get_history(0, 0)
vel_vert = rocket.get_history(1, 1)
acc_vert = rocket.get_history(2, 1)
pos_mass = rocket.get_history(4, 0)
thrust_x = rocket.get_history(5, 0)
thrust_y = rocket.get_history(5, 1)
orient_z = rocket.get_history(3, 2)
orient_y = rocket.get_history(3, 1)
orient_x = rocket.get_history(3, 0)

plt.figure()
plt.plot(pos_height, label="height")
plt.plot(vel_vert, label="vertical velocity")
plt.plot(acc_vert, label="vertical acceleration")
plt.plot(pos_mass, label="mass")
plt.legend()
plt.savefig("sim_data.png")
plt.close()

plt.figure()
plt.scatter(pos_x, pos_height, label="trajectory")
plt.legend()
plt.savefig("traj.png")
plt.close()

plt.figure()
plt.plot(thrust_x, label="x")
plt.plot(thrust_y, label="y")
plt.legend()
plt.savefig("thrust_data.png")
plt.close()

plt.figure()
plt.plot(orient_x, label="x axis")
plt.plot(orient_y, label="y axis")
plt.plot(orient_z, label="z axis")
plt.legend()
plt.savefig("orientation_data.png")
plt.close()
