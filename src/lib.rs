use pyo3::prelude::*;

// cmd /k .\.venv\Scripts\activate.bat

/// A Python module implemented in Rust.
#[pymodule]
mod rocket_sim {
    use pyo3::prelude::*;

    const G: f32 = -9.81;

    #[derive(Copy, Clone, Debug)]
    struct Vec3f {
        x: f32,
        y: f32,
        z: f32,
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

    impl Vec3f {
        pub fn new() -> Self {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
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
        mass: f32,
        dry_mass: f32,
        dm: f32,
        thrust: f32,
        dt: f32,
        dur: i32,
        state_history: Vec<RocketState>,
        powered: bool,
    }

    use pyo3::exceptions::PyValueError;
    #[pymethods]
    impl Rocket {
        #[new]
        pub fn new(mass: f32, dry_mass: f32, dm: f32, thrust: f32, dt: f32, dur: i32) -> Self {
            Self {
                ang: Vec3f {
                    x: PI / 2.0,
                    y: 0.0,
                    z: 0.0,
                },
                pos: Vec3f::new(),
                vel: Vec3f::new(),
                acc: Vec3f::new(),
                mass,
                dry_mass,
                dm,
                thrust,
                dt,
                dur,
                state_history: Vec::new(),
                powered: true,
            }
        }

        pub fn print(&self) {
            println!(
                "{:?} | {:?} | {:?} | {:?}",
                self.pos, self.vel, self.acc, self.mass,
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
                    y: G * self.mass,
                    z: 0.0,
                };

                let thrust_force = if self.powered {
                    Vec3f {
                        x: self.thrust * self.ang.x.cos(),
                        y: self.thrust * self.ang.y.cos(),
                        z: 0.0,
                    }
                } else {
                    Vec3f::new()
                };

                self.acc = (grav_force + thrust_force) * (1.0 / self.mass);
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
                    mass: self.mass,
                    thrust: self.thrust,
                    time: i as f32 * self.dt,
                });

                if self.mass <= self.dry_mass {
                    self.powered = false;
                }

                if self.powered {
                    self.mass -= self.dm * self.dt;
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
