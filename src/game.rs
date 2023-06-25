use derive_getters::Getters;
use rand::{rngs::ThreadRng, Rng};
use three_d::*;

#[derive(Getters, Default)]
pub struct Game {
    field: Field,
    balls: Vec<Ball>,
}

impl Game {
    pub fn put_ball(&mut self) {
        let ball = Ball {
            center_position: vec3(self.field.rng.gen::<f32>() * 1.0 - 0.5 , 1.0, self.field.rng.gen::<f32>() * 1.0 - 0.5),
            ..Default::default()
        };

        self.balls.push(ball);
    }

    pub fn do_frame(&mut self) {
        for ball in &mut self.balls {
            ball.do_frame(&self.field);
        }
    }
    
    pub fn do_collision(&mut self) {
        let ball_clones: Vec<Ball> = self.balls.to_vec();
        for ball in &mut self.balls {
            for other in &ball_clones {
                ball.do_collision(other, &self.field)
            }
        }
    }

    pub fn to_gm(&self, context: &Context) -> Vec<Gm<Mesh, PhysicalMaterial>> {
        let mut fields = vec![self.field.to_gm(context)];
        let mut balls = self.balls.iter().map(|x| x.to_gm(context)).collect();
        
        let mut return_vec = vec![];
        return_vec.append(&mut fields);
        return_vec.append(&mut balls);
        
        return_vec
    }
}


#[derive(Getters)]
pub struct Field {
    x_min: f32,
    x_max: f32,
    z_min: f32,
    z_max: f32,
    y_min: f32,
    y_max: f32,
    gravitational_acceleration: f32,
    elasticity: f32,
    rng: ThreadRng,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            x_min: -1.0, x_max: 1.0,
            y_min: -1.0, y_max: 1.0,
            z_min: -1.0, z_max: 1.0,
            gravitational_acceleration: 0.0001,
            elasticity: 0.8,
            rng: rand::thread_rng(),
        }
    }
}

impl Field {
    fn to_gm(&self, context: &Context) -> Gm<Mesh, PhysicalMaterial> {
        let mut cube = Gm::new(
            Mesh::new(context, &CpuMesh::cube()),
            PhysicalMaterial::new_transparent(
                context,
                &CpuMaterial {
                    albedo: Color {
                        r: 128,
                        g: 128,
                        b: 128,
                        a: 128,
                    },
                    ..Default::default()
                },
            ),
        );

        // FIX: not mid pattern
        let translation = Mat4::from_translation(
            vec3(0.0, 0.0, 0.0)
        );
        let scale = Mat4::from_nonuniform_scale(
            self.x_max - self.x_min,
            self.y_max - self.y_min,
            self.z_max - self.z_min,
        );
        let half = Mat4::from_scale(0.5);

        cube.set_transformation(translation * half * scale);
        
        cube
    }
}

#[derive(Getters, Clone)]
pub struct Ball {
    center_position: Vector3<f32>,
    velocity: Vector3<f32>,
    radius: f32
}

impl Default for Ball {
    fn default() -> Self {
        Self { center_position: vec3(0.0, 1.0, 0.0), radius: 0.1, velocity: vec3(0.0, 0.0, 0.0) }
    }
}

impl Ball {
    fn do_frame(&mut self, field: &Field) {
        self.velocity += vec3(0.0, -field.gravitational_acceleration(), 0.0);
        self.center_position += self.velocity;

        if self.center_position.x < *field.x_min() || self.center_position.x > *field.x_max() {
            self.velocity = vec3(-self.velocity.x * field.elasticity, self.velocity.y, self.velocity.z);
        }

        if self.center_position.z < *field.z_min() || self.center_position.z > *field.z_max() {
            self.velocity = vec3(self.velocity.x, self.velocity.y, -self.velocity.z * field.elasticity);
        }

        if self.center_position.y < *field.y_min() {
            self.velocity = vec3(self.velocity.x, -self.velocity.y * field.elasticity, self.velocity.z);
        }
    }
    
    fn do_collision(&mut self, other: &Self, field: &Field) {
        if self.center_position == other.center_position {
            // TODO: change another method
            return;
        }

        let direction = self.center_position - other.center_position;
        let radius_add = self.radius + other.radius;
        
        if direction.dot(direction) <= radius_add * radius_add {
            self.velocity = Self::collision(self.velocity, direction, field.elasticity)
        }
    }
    
    fn collision(velocity: Vector3<f32>, plane_direction: Vector3<f32>, elasticity: f32) -> Vector3<f32> {
        let plane_direction = plane_direction.normalize();

        // divide to plane direction vector and other vector
        let dot = velocity.dot(plane_direction);

        // avoid serial collision
        if dot >= 0.0 {
            return velocity;
        }

        let a = plane_direction * velocity.dot(plane_direction);
        let b = velocity - a;

        // reverse a
        b - a * elasticity
    }

    fn to_gm(&self, context: &Context) -> Gm<Mesh, PhysicalMaterial> {
        let mut sphere = Gm::new(
            Mesh::new(context, &CpuMesh::sphere(16)),
            PhysicalMaterial::new_transparent(
                context,
                &CpuMaterial {
                    albedo: Color {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 200,
                    },
                    ..Default::default()
                },
            ),
        );

        sphere.set_transformation(Mat4::from_translation(*self.center_position()) * Mat4::from_scale(*self.radius()));
        
        sphere
    }
}

