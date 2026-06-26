use pyo3::prelude::*;

// cmd /k .\.venv\Scripts\activate.bat

/// A Python module implemented in Rust.
#[pymodule]
mod rocket_sim {
    use pyo3::prelude::*;

    const G: f32 = -9.81;
    const FLUID_DENSITY: f32 = 1.225;

    #[derive(Copy, Clone, Debug)]
    #[pyclass] // pyo3 attribute for classes usable in python
    // Basic 3 Dimensional Vector used throughout the simulator
    struct Vec3f {
        x: f32, // x component
        y: f32, // y component
        z: f32, // z component
    }

    #[pymethods] // pyo3 attribute for methods usable in python
    impl Vec3f {
        #[new] // defualt constructor attribute for pyo3
        // struct initializer
        pub fn new() -> Self {
            Self { 
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }

        // used to redefine the struct from python
        pub fn redefine(&mut self, a: f32, b: f32, c: f32) {
            *self = Self {x: a, y: b, z: c}; // had to dereference self. Idk what that does, the compiler just told me to do it. I should probably look into that
        }
    }

    use std::num::NonZeroI32;
// add F32 to struct Vec3f
    use std::ops::Add;
    impl Add<f32> for Vec3f {
        // Allows the operation for adding to be done with the Vec3f Struct
        // I lowkirkenuinly am not completely sure how this works, I just pulled it from the rust book
        type Output = Vec3f; // specifying the ouput type for the function below. I think something to do with like <T> or smth


        fn add(self, other: f32) -> Self::Output {
            Vec3f {
                x: self.x + other,
                y: self.y + other,
                z: self.z + other,
            }
        }
    }

    // add Vec3f to struct Vec3f
    impl Add<Vec3f> for Vec3f {
        type Output = Vec3f;

        fn add(self, other: Vec3f) -> Self::Output {
            Vec3f {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    use std::ops::Sub;
    impl Sub<Vec3f> for Vec3f {
        type Output = Vec3f;

        fn sub(self, other: Vec3f) -> Self::Output {
            Vec3f {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    // do like the += thingy to Vec3f with another Vec3f
    use std::ops::AddAssign;
    impl AddAssign<Vec3f> for Vec3f {
        fn add_assign(&mut self, other: Vec3f) {
            self.x += other.x;
            self.y += other.y;
            self.z += other.z;
        }
    }

    // like the above but with type F32
    impl AddAssign<f32> for Vec3f {
        fn add_assign(&mut self, other: f32) {
            self.x += other;
            self.y += other;
            self.z += other;
        }
    }

    // multiply Vec3f by f32
    use std::ops::Mul;
    impl Mul<f32> for Vec3f {
        type Output = Vec3f;

        fn mul(self, other: f32) -> Self::Output {
            Vec3f {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other,
            }
        }
    }

    // divide Vec3f by f32
    use std::ops::Div;
    impl Div<f32> for Vec3f {
        type Output = Vec3f;

        fn div(self, other: f32) -> Self::Output {
            Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            }
        }
    }

    // do like the *= thingy with Vec3f and f32
    use std::ops::MulAssign;
    impl MulAssign<f32> for Vec3f {
        fn mul_assign(&mut self, other: f32) {
            self.x *= other;
            self.y *= other;
            self.z *= other;
        }
    }

    // define cross product method for Vec3f
    impl Vec3f {
        pub fn refcross(first: &Vec3f, second: &Vec3f) -> Vec3f {
            Vec3f {
                x: first.y * second.z - first.z * second.y,
                y: first.z * second.x - first.x * second.z,
                z: first.x * second.y - first.y * second.x,
            }
        }

        pub fn magnitude(&self) -> f32 { // takes the magnitude of a Vec3f struct
                f32::sqrt(
                self.x.powi(2) + 
                self.y.powi(2) + 
                self.z.powi(2)
            )
        }
    }

    #[pyclass]
    struct PidController {
        kp: f32,
        kd: f32,
        ki: f32,
        integral: f32,
        derivative: f32,
        target: f32,
        prev_err: f32,
    }

    #[pymethods]
    impl PidController {
        #[new]
        pub fn new(kp: f32, kd: f32, ki: f32, target: f32) -> Self {
            Self {
                kp,
                kd,
                ki,
                target,
                prev_err: target,
                derivative: 0.0,
                integral: 0.0,
            }
        }

        pub fn run_pid(&mut self, reference: f32, dt: f32) -> f32 {
            self.integral += reference * dt;
            self.derivative = (reference - self.prev_err) / dt;
            self.prev_err = reference;
            (-0.00001 * self.kp * reference)
            + (-0.00001 * self.integral * self.ki)
            + (-0.00001 * self.derivative * self.kd)
        }
    }

    #[pyclass]
    #[derive(Clone, Copy)]
    // input struct to make the initialization of rocket more clean. This is initialized in python
    struct MassStruct {
        dry_mass: f32, // mass without fuel
        wet_mass: f32, // mass with fuel
        mass: f32, // current mass of the rocket. This gets changed throughout the simulation
    }

    // constructor for the mass struct that can be utilized in python
    #[pymethods]
    impl MassStruct {
        #[new]
        pub fn new(dry_mass: f32, wet_mass: f32) -> Self {
            Self {
                dry_mass,
                wet_mass,
                mass: wet_mass,
            }
        }
    }

    // struct for documentation purposes. Can be put in a vector to store the state of the simulation at every time step
    #[derive(Clone, Debug)]
    struct RocketState {
        ang: Vec3f, // orientation vector of the rocket relative to it's spacial coordinates
        pos: Vec3f, // position vector of the rocket relative to it's spactial coordinates
        vel: Vec3f, // velocity vector of the rocket relative to it's spacial coordiantes
        acc: Vec3f, // acceleration vector of the rocket relateive to it's spacial coordinates
        mass: f32, // captured mass of the rocket
        thrust: f32, // current thrust of the rocket
        time: f32, // current time step NOT IN SECONDS
        thrust_vec: Vec3f, // the unit vector of the rocket engine relative to the rocket body
        drag: f32,
    }

    // like the mass struct but stores all the information pertaining to rotation
    #[derive(Clone, Copy)]
    #[pyclass]
    struct RotateStruct {
        cp: f32, // center of pressure from top
        cg: f32, // center of gravity from top
        cmp: f32, // point of pressure from motor from top // drag coefficient of the top of the rocket. Usually a cone
        body_vector: Vec3f, // vector used for torque calculations
        dimensions: Vec3f,  // dimensions of the rocket
        inertia_moment: f32, // moment of inertia in the horizontal axis. Probably should make this a Vec3f
        angular_vel: Vec3f, // angular velocity vector of the rocket
        angular_acc: Vec3f, // angular acceleratio vector of the rocket
        rotational_drag: Vec3f, // rotational drag force vector: currently unused
        origin: Vec3f, // origin of rotation of the rocket. Basically just the CG relative to the body dimensions
        dampening_constant: f32, // the constant that determines the stability of the rocket. Keep this lower for more unstable systems.
    }
    
    // along with the python constructor method, it also has a method to simulate the rotation of the rocket
    use pyo3::exceptions::PyValueError;
    #[pymethods]
    impl RotateStruct {
        #[new]
        pub fn new(
            cp: f32,
            cg: f32,
            cmp: f32, 
            dimensions: Vec3f,
            mass: MassStruct,
            dampening_constant: f32,
        ) -> Self {
            Self {
                cp,
                cg,
                cmp,
                dimensions,
                inertia_moment: (1.0 / 12.0) * mass.mass * (dimensions.x.powi(2) + dimensions.y.powi(2)), // moment of intertia formula for a cylinder
                angular_vel: Vec3f::new(),
                angular_acc: Vec3f::new(),
                rotational_drag: Vec3f::new(),
                body_vector: Vec3f { // calcs the body vector, used for torque, as the moment between the location of pressure from the motor and the center of gravity
                    x: 0.0,
                    y: cg - cmp,
                    z: 0.0,
                },
                origin: Vec3f { // sets the origin of the rocket on which it'll rotate
                    x: dimensions.x / 2.0,
                    y: dimensions.y - cg,
                    z: dimensions.z / 2.0
                },
                dampening_constant,
            }
        }

        // method that'll simulate the rotational phsics of the rocket
        pub fn rotate_physics(&mut self, orientation_vector: &mut Vec3f, thrust_vector: &Vec3f, thrust: f32, dt: f32) {
            let force = *thrust_vector * thrust; // total force vector, multiplying the unit vector known as thrust_vector by the actual thrust
            let torque = Vec3f::refcross(&self.body_vector, &force); // takes the torque between the rocket body and the thrust force
            let drag_torque = self.angular_vel * - self.dampening_constant; // 0.001 is the dampening constant K. Changing this will change the stability fo the rocket
            let total_torque = torque + drag_torque;
            self.angular_acc = total_torque / self.inertia_moment; // gets angular acceleration
            self.angular_vel += self.angular_acc * dt; // then gets angular velocity
            *orientation_vector += self.angular_vel * dt; // changes the orientation vector passed in, in this case, Rocket's angle vector ang
        }
    }

    use std::f32::consts::PI; // imports PI because I'm too lazy to type it out

    #[pyclass]
    // struct of the main rocket. This is where everything is brought together >:)
    struct Rocket {
        ang: Vec3f, // angle vector of the rocket relative to spatial coordinates
        pos: Vec3f, // position vector of the rocket relative to spatial coordinates
        vel: Vec3f, // velocity vector of the rocket relative to spatial coordinates
        acc: Vec3f, // acceleration vector of the rocket relative to spatial coordinates
        rotational: RotateStruct, // rotational struct for the rocket; handles all the rotational stuff
        mass: MassStruct, // mass struct of the rocket, handles most of the mass stuff
        dm: f32, // change in mass for the rocket. I probably should have put this in the mass struct but it's a bit too late now for me to care
        thrust: f32, // thrust of the rocket motor. This will likely be changed later to a dynamic engine performance chart
        dt: f32, // time increment of the simulation
        dur: i32, // duration of the simulation in total steps that are to be taken
        state_history: Vec<RocketState>, // history vector of the simulation, used for logging purposes
        powered: bool, // bool for whether the rocket is powered or not. Just used for organization purposes
        thrust_vec: Vec3f, // unit vector for the direction of the rocket engine relative to the rocket body
        drag_coefficient: f32,
    }

    #[pymethods]
    impl Rocket {
        // Rocket constructor
        #[new]
        pub fn new(
            mass: MassStruct,
            dm: f32,
            rotate: RotateStruct,
            thrust: f32,
            dt: f32,
            dur: i32,
            thrust_vec: Vec3f,
            drag_coefficient: f32
        ) -> Self {
            Self {
                ang: Vec3f {
                    x: (PI / 2.0),
                    y: 0.0,
                    z: 0.0,
                },
                pos: Vec3f::new(),
                vel: Vec3f::new(),
                acc: Vec3f::new(),
                mass,
                dm,
                thrust,
                dt,
                dur,
                state_history: Vec::new(),
                powered: true,
                rotational: rotate,
                thrust_vec,
                drag_coefficient,
            }
        }

        //  used for debugging purposes, although I hate how #[derive(Debug)] prints stuff. It looks so ugly and cluttered
        pub fn print(&self) {
            println!(
                "{:?} | {:?} | {:?} | {:?}",
                self.pos, self.vel, self.acc, self.mass.mass,
            )
        }


        // this is where all the translational magic happens
        pub fn uncontrolled_sim(&mut self) {
            let mut cycle_tracker: i32 = 0;
            for i in 0..self.dur { // sets the duration to cycle 0 through the defined duration of the simulation
                cycle_tracker += 1;
                if self.pos.y < 0.0 { // tests whether the rocket hit the ground. If so, it ends the simulation early
                    break;
                }

                let grav_force = Vec3f { // sets the force imposed by gravity. Since this is always constant, I decided to separate it from the thrust vectors which are not
                    x: 0.0,
                    y: G * self.mass.mass,
                    z: 0.0,
                };
                
                let forward_vec = Vec3f { // although not always in use because the motor isn't always on, I decided to always define the forward facing vector of the rocket in case I decide to implement parachutes later on
                    x: self.ang.z.sin(),
                    y: self.ang.z.cos(),
                    z: 0.0,
                };

                let summed_velocity: f32 = self.vel.magnitude();
                let reference_area: f32 = (self.rotational.dimensions.x / 2.0).powi(2) * PI;
                let drag = 0.5 * self.drag_coefficient * FLUID_DENSITY * reference_area * summed_velocity.powi(2);

                let drag_force = if summed_velocity > 0.0001 {
                    Vec3f {
                        x: -self.vel.x / summed_velocity * drag,
                        y: -self.vel.y / summed_velocity * drag,
                        z: -self.vel.z / summed_velocity * drag,
                    }
                } else {
                    Vec3f::new()
                };

                let thrust_force = if self.powered { // thrust force is only calculated if the powered boolean is true
                    Vec3f {
                        x: forward_vec.x * (self.thrust),
                        y: forward_vec.y * (self.thrust),
                        z: forward_vec.z * (self.thrust),
                    }
                } else { // if not, it just sets it to the defualt vector values, which are all 0
                    Vec3f::new()
                };

                self.acc = (grav_force + thrust_force + drag_force) / self.mass.mass; // calculated the acceleration vector of the rocket body
                self.vel += self.acc * self.dt; // calculates the velocity vector of the rocket body
                self.pos += self.vel * self.dt; // calcuatles the position vector of the rocket body

                println!("{:?}", self.ang);

                self.rotational.rotate_physics(&mut self.ang, &mut self.thrust_vec, self.thrust, self.dt); // calls the rotation simulation method defined above

                self.state_history.push(RocketState { // pushes rocket state to the state history vector
                    ang: self.ang,
                    pos: self.pos,
                    vel: self.vel,
                    acc: self.acc,
                    mass: self.mass.mass,
                    thrust: self.thrust,
                    time: i as f32 * self.dt,
                    thrust_vec: self.thrust_vec,
                    drag,
                });

                // checks if the rocket still has any fuel left. If it doesn't than the rocket power is set to false
                if self.mass.mass <= self.mass.dry_mass {
                    self.powered = false;
                }

                // decreases the rocket's mass based on fuel consumption. If unpowered, this block does not run
                if self.powered {
                    self.mass.mass -= self.dm * self.dt;
                }
            }
            println!("Total Cycles: {}", cycle_tracker);
        }

        // this method is used for printing all the relative translational information of the rocket. It's kind of outdated at the moment so...
        pub fn print_history(&self) {
            for i in 0..self.state_history.len() {
                println!(
                    "
                ang {} | {} | {}\n
                pos {} | {} | {}\n
                vel {} | {} | {}\n
                acc {} | {} | {}\n
                mass {}\n
                thrust {}\n
                time {}\n\n\n
                ",
                    self.state_history[i].ang.x,
                    self.state_history[i].ang.y,
                    self.state_history[i].ang.z,
                    self.state_history[i].pos.x,
                    self.state_history[i].pos.y,
                    self.state_history[i].pos.z,
                    self.state_history[i].vel.x,
                    self.state_history[i].vel.y,
                    self.state_history[i].vel.z,
                    self.state_history[i].acc.x,
                    self.state_history[i].acc.y,
                    self.state_history[i].acc.z,
                    self.state_history[i].mass,
                    self.state_history[i].thrust,
                    self.state_history[i].time,
                );
            }
        }

        // now this is the method that I'm most proud of in this entire project. It can be used to fetch the history information of anything stored within the rocket state
        pub fn get_history(&self, tip: i8, id: i8) -> PyResult<Vec<f32>> { // it just takes 2 things, the type ID represented as tip (I couldn't think of a better name), and the index of the vector being retrieved represented by id
            match tip {
                0 => match id {
                    0 => Ok(self.state_history.iter().map(|s| s.pos.x).collect()),
                    1 => Ok(self.state_history.iter().map(|s| s.pos.y).collect()),
                    2 => Ok(self.state_history.iter().map(|s| s.pos.z).collect()),
                    _ => Err(PyValueError::new_err("Invalid get_history fetch ID"))?,
                },
                1 => match id {
                    0 => Ok(self.state_history.iter().map(|s| s.vel.x).collect()),
                    1 => Ok(self.state_history.iter().map(|s| s.vel.y).collect()),
                    2 => Ok(self.state_history.iter().map(|s| s.vel.z).collect()),
                    _ => Err(PyValueError::new_err("Invalid get_history fetch ID"))?,
                },
                2 => match id {
                    0 => Ok(self.state_history.iter().map(|s| s.acc.x).collect()),
                    1 => Ok(self.state_history.iter().map(|s| s.acc.y).collect()),
                    2 => Ok(self.state_history.iter().map(|s| s.acc.z).collect()),
                    _ => Err(PyValueError::new_err("Invalid get_history fetch ID"))?,
                },
                3 => match id {
                    0 => Ok(self.state_history.iter().map(|s| s.ang.x * 57.295).collect()),
                    1 => Ok(self.state_history.iter().map(|s| s.ang.y * 57.295).collect()),
                    2 => Ok(self.state_history.iter().map(|s| s.ang.z * 57.295).collect()),
                    _ => Err(PyValueError::new_err("Invalid get_history fetch ID"))?,
                },
                4 => Ok(self.state_history.iter().map(|s| s.mass).collect()),
                5 => match id {
                    0 => Ok(self.state_history.iter().map(|s| s.thrust_vec.x).collect()),
                    1 => Ok(self.state_history.iter().map(|s| s.thrust_vec.y).collect()),
                    2 => Ok(self.state_history.iter().map(|s| s.thrust_vec.z).collect()),
                    _ => Err(PyValueError::new_err("Invalid get_history fetch ID"))?,
                }
                6 => Ok(self.state_history.iter().map(|s| s.drag).collect()),
                _ => Err(PyValueError::new_err("Invalid get_history fetch type")),
            }
        }
    }
}
