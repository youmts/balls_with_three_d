use derive_getters::Getters;
use three_d::*;

#[derive(Getters, Default)]
pub struct Game {
    field: Field,
    balls: Vec<Ball>,
}

impl Game {
    pub fn put_ball(&mut self) {
        let ball = Ball {
            ..Default::default()
        };

        self.balls.push(ball);
    }

    pub fn do_frame(&mut self) {
        for ball in &mut self.balls {
            ball.do_frame(&self.field);
        }
    }

    pub fn to_gm(&self, context: &Context) -> Vec<Gm<Mesh, PhysicalMaterial>> {
        self.balls.iter().map(|x| x.to_gm(context)).collect()
    }
}


#[derive(Getters)]
pub struct Field {
    x_min: f32,
    x_max: f32,
    z_min: f32,
    z_max: f32,
    y_min: f32,
    gravitational_acceleration: f32
}

impl Default for Field {
    fn default() -> Self {
        Self { x_min: -1.0, x_max: 1.0, y_min: -1.0, z_min: -1.0, z_max: 1.0, gravitational_acceleration: 0.01 }
    }
}

#[derive(Getters)]
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
            self.velocity = vec3(-self.velocity.x, self.velocity.y, self.velocity.z);
        }

        if self.center_position.z < *field.z_min() || self.center_position.z > *field.z_max() {
            self.velocity = vec3(self.velocity.x, self.velocity.y, -self.velocity.z);
        }

        if self.center_position.y < *field.y_min() {
            self.velocity = vec3(self.velocity.x, -self.velocity.y, self.velocity.z);
        }
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

