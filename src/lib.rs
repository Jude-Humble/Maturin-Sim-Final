use pyo3::prelude::*;

// cmd /k .\.venv\Scripts\activate.bat

/// A Python module implemented in Rust.
#[pymodule]
mod rocket_sim {
    use pyo3::prelude::*;

    const G: f32 = -9.81;

    #[derive(Copy, Clone, Debug)]
    #[pyclass]
    struct Vec3f {
        x: f32,
        y: f32,
        z: f32,
    }

    #[pymethods]
    impl Vec3f {
        #[new]
        pub fn new() -> Self {
            Self { 
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }

        pub fn redefine(&mut self, a: f32, b: f32, c: f32) {
            *self = Self {x: a, y: b, z: c};
        }
    }

    use std::ops::Add;
    impl Add<f32> for Vec3f {
        type Output = Vec3f;

        fn add(self, rhs: f32) -> Self::Output {
            Vec3f {
                x: self.x + rhs,
                y: self.y + rhs,
                z: self.z + rhs,
            }
        }
    }

    impl Add<Vec3f> for Vec3f {
        type Output = Vec3f;

        fn add(self, rhs: Vec3f) -> Self::Output {
            Vec3f {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    use std::ops::AddAssign;
    impl AddAssign<Vec3f> for Vec3f {
        fn add_assign(&mut self, rhs: Vec3f) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
        }
    }

    impl AddAssign<f32> for Vec3f {
        fn add_assign(&mut self, rhs: f32) {
            self.x += rhs;
            self.y += rhs;
            self.z += rhs;
        }
    }

    use std::ops::Mul;
    impl Mul<f32> for Vec3f {
        type Output = Vec3f;

        fn mul(self, rhs: f32) -> Self::Output {
            Vec3f {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }

    use std::ops::Div;
    impl Div<f32> for Vec3f {
        type Output = Vec3f;

        fn div(self, rhs: f32) -> Self::Output {
            Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            }
        }
    }

    use std::ops::MulAssign;
    impl MulAssign<f32> for Vec3f {
        fn mul_assign(&mut self, rhs: f32) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
        }
    }

    impl Vec3f {
        pub fn refcross(lhs: &Vec3f, rhs: &Vec3f) -> Vec3f {
            Vec3f {
                x: lhs.y * rhs.z - lhs.z * rhs.y,
                y: lhs.z * rhs.x - lhs.x * rhs.z,
                z: lhs.x * rhs.y - lhs.y * rhs.x,
            }
        }
    }

    #[pyclass]
    #[derive(Clone, Copy)]
    struct MassStruct {
        dry_mass: f32,
        wet_mass: f32,
        mass: f32,
    }

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

    #[derive(Clone, Debug)]
    struct RocketState {
        ang: Vec3f,
        pos: Vec3f,
        vel: Vec3f,
        acc: Vec3f,
        mass: f32,
        thrust: f32,
        time: f32,
    }

    #[derive(Clone, Copy)]
    #[pyclass]
    struct RotateStruct {
        cp: f32,
        cg: f32,
        cmp: f32,
        drag_coefficient: f32,
        body_vector: Vec3f,
        dimensions: Vec3f,
        inertia_moment: f32,
        angular_vel: Vec3f,
        angular_acc: Vec3f,
        rotational_drag: Vec3f,
        origin: Vec3f,
    }

    use pyo3::exceptions::PyValueError;
    #[pymethods]
    impl RotateStruct {
        #[new]
        pub fn new(
            cp: f32,
            cg: f32,
            cmp: f32,
            drag_coefficient: f32,
            dimensions: Vec3f,
            mass: MassStruct,
        ) -> Self {
            Self {
                cp,
                cg,
                cmp,
                drag_coefficient,
                dimensions,
                inertia_moment: (1.0 / 12.0)
                    * mass.mass
                    * (dimensions.x.powi(2) + dimensions.y.powi(2)),
                angular_vel: Vec3f::new(),
                angular_acc: Vec3f::new(),
                rotational_drag: Vec3f::new(),
                body_vector: Vec3f {
                    x: 0.0,
                    y: cg - cmp,
                    z: 0.0,
                },
                origin: Vec3f {
                    x: dimensions.x / 2.0,
                    y: dimensions.y - cg,
                    z: dimensions.z / 2.0
                }
            }
        }

        pub fn rotate_physics(&mut self, orientation_vector: &mut Vec3f, thrust_vector: &Vec3f, dt: f32) {
            let torque = Vec3f::refcross(&self.body_vector, thrust_vector);
            self.angular_acc = torque / self.inertia_moment;
            self.angular_vel += self.angular_acc * dt;
            self.angular_vel = self.angular_vel * 0.98;
            *orientation_vector += self.angular_vel * dt;
        }
    }

    use std::f32::consts::PI;
    /*impl RocketState {
        pub fn new() -> Self {
            Self {
                ang: Vec3f {
                    x: PI / 2.0,
                    y: 0.0,
                    z: 0.0,
                },
                pos: Vec3f::new(),
                vel: Vec3f::new(),
                acc: Vec3f::new(),
                mass: 0.0,
                thrust: 0.0,
                time: 0.0,
            }
        }
    }*/

    #[pyclass]
    struct Rocket {
        ang: Vec3f,
        pos: Vec3f,
        vel: Vec3f,
        acc: Vec3f,
        rotational: RotateStruct,
        mass: MassStruct,
        dm: f32,
        thrust: f32,
        dt: f32,
        dur: i32,
        state_history: Vec<RocketState>,
        powered: bool,
        thrust_vec: Vec3f,
    }

    #[pymethods]
    impl Rocket {
        #[new]
        pub fn new(
            mass: MassStruct,
            dm: f32,
            rotate: RotateStruct,
            thrust: f32,
            dt: f32,
            dur: i32,
        ) -> Self {
            Self {
                ang: Vec3f {
                    x: (PI / 2.0) - 0.1,
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
                thrust_vec: Vec3f::new(),
            }
        }

        pub fn print(&self) {
            println!(
                "{:?} | {:?} | {:?} | {:?}",
                self.pos, self.vel, self.acc, self.mass.mass,
            )
        }

        /*#[staticmethod]
        pub fn rand_pos_vec() -> PyResult<Vec<i32>> {
            use rand::Rng;
            let mut utvec: Vec<i32> = vec![0];
            let mut rng = rand::thread_rng();
            for i in 1..10 {
                utvec.push(rng.gen_range(0..=50));
            }
            Ok(utvec)
        }*/

        pub fn uncontrolled_sim(&mut self) {
            for i in 0..self.dur {
                if self.pos.y < 0.0 {
                    break;
                }

                let grav_force = Vec3f {
                    x: 0.0,
                    y: G * self.mass.mass,
                    z: 0.0,
                };

                let forward_vec = Vec3f {
                    x: self.ang.x.cos() * self.ang.y.cos(),
                    y: self.ang.x.sin(),
                    z: self.ang.x.cos() * self.ang.y.sin(),
                };

                let thrust_force = if self.powered {
                    Vec3f {
                        x: forward_vec.x * self.thrust,
                        y: forward_vec.y * self.thrust,
                        z: forward_vec.z * self.thrust,
                    }
                } else {
                    Vec3f::new()
                };

                self.acc = (grav_force + thrust_force) * (1.0 / self.mass.mass);
                self.vel += self.acc * self.dt;
                self.pos += self.vel * self.dt;

                /*
                self.ang = Vec3f {
                    x: self.vel.y.atan2(self.vel.x),
                    y: self.vel.x.atan2(self.vel.y),
                    z: 0.0,
                };*/

                self.state_history.push(RocketState {
                    ang: self.ang,
                    pos: self.pos,
                    vel: self.vel,
                    acc: self.acc,
                    mass: self.mass.mass,
                    thrust: self.thrust,
                    time: i as f32 * self.dt,
                });

                if self.mass.mass <= self.mass.dry_mass {
                    self.powered = false;
                }

                if self.powered {
                    self.mass.mass -= self.dm * self.dt;
                }
            }
        }

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

        pub fn get_history(&self, tip: i8, id: i8) -> PyResult<Vec<f32>> {
            match tip {
                0 => match id {
                    0 => Ok(self.state_history.iter().map(|s| s.pos.x).collect()),
                    1 => Ok(self.state_history.iter().map(|s| s.pos.y).collect()),
                    2 => Ok(self.state_history.iter().map(|s| s.pos.z).collect()),
                    _ => Err(PyValueError::new_err("Invalid get_history fetch ID")),
                },
                1 => match id {
                    0 => Ok(self.state_history.iter().map(|s| s.vel.x).collect()),
                    1 => Ok(self.state_history.iter().map(|s| s.vel.y).collect()),
                    2 => Ok(self.state_history.iter().map(|s| s.vel.z).collect()),
                    _ => Err(PyValueError::new_err("Invalid get_history fetch ID")),
                },
                2 => match id {
                    0 => Ok(self.state_history.iter().map(|s| s.acc.x).collect()),
                    1 => Ok(self.state_history.iter().map(|s| s.acc.y).collect()),
                    2 => Ok(self.state_history.iter().map(|s| s.acc.z).collect()),
                    _ => Err(PyValueError::new_err("Invalid get_history fetch ID")),
                },
                3 => match id {
                    0 => Ok(self.state_history.iter().map(|s| s.ang.x).collect()),
                    1 => Ok(self.state_history.iter().map(|s| s.ang.y).collect()),
                    2 => Ok(self.state_history.iter().map(|s| s.ang.z).collect()),
                    _ => Err(PyValueError::new_err("Invalid get_history fetch ID")),
                },
                4 => Ok(self.state_history.iter().map(|s| s.mass).collect()),
                _ => Err(PyValueError::new_err("Invalid get_history fetch type")),
            }
        }
    }

    #[pyfunction]
    fn def_vec() -> PyResult<Vec<i32>> {
        Ok(vec![0, 0, 0])
    }
}
