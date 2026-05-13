use crate::engine::utils::vector::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Collider {
    pub position: Vector2,
    pub width: f32,
    pub height: f32,
}

impl Collider {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vector2::new(x, y),
            width,
            height,
        }
    }

    pub fn is_colliding(&self, other: &Collider) -> bool {
        self.position.x < other.position.x + other.width &&
        self.position.x + self.width > other.position.x &&
        self.position.y < other.position.y + other.height &&
        self.position.y + self.height > other.position.y
    }
}

#[derive(Debug, Clone)]
pub struct PhysicsBody {
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
    pub mass: f32,
    pub collider: Option<Collider>,
}

impl PhysicsBody {
    pub fn new(position: Vector2, mass: f32) -> Self {
        Self {
            position,
            velocity: Vector2::zero(),
            acceleration: Vector2::zero(),
            mass: if mass <= 0.0 { 1.0 } else { mass },
            collider: None,
        }
    }

    pub fn with_collider(mut self, width: f32, height: f32) -> Self {
        self.collider = Some(Collider::new(self.position.x, self.position.y, width, height));
        self
    }

    pub fn apply_force(&mut self, force: Vector2) {
        // F = ma => a = F/m
        let force_accel = force * (1.0 / self.mass);
        self.acceleration += force_accel;
    }

    pub fn apply_gravity(&mut self, gravity_strength: f32) {
        self.apply_force(Vector2::new(0.0, gravity_strength * self.mass));
    }

    pub fn update(&mut self, delta_time: f32) {
        // Euler integration
        self.velocity += self.acceleration * delta_time;
        self.position += self.velocity * delta_time;
        
        // Update collider position if it exists
        if let Some(ref mut collider) = self.collider {
            collider.position = self.position;
        }

        // Reset acceleration for the next frame
        self.acceleration = Vector2::zero();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_math() {
        let v1 = Vector2::new(10.0, 5.0);
        let v2 = Vector2::new(2.0, 3.0);
        
        assert_eq!(v1 + v2, Vector2::new(12.0, 8.0));
        assert_eq!(v1 - v2, Vector2::new(8.0, 2.0));
        assert_eq!(v1 * 2.0, Vector2::new(20.0, 10.0));
    }

    #[test]
    fn test_collision() {
        let c1 = Collider::new(0.0, 0.0, 10.0, 10.0);
        let c2 = Collider::new(5.0, 5.0, 10.0, 10.0);
        let c3 = Collider::new(20.0, 20.0, 10.0, 10.0);
        
        assert!(c1.is_colliding(&c2));
        assert!(!c1.is_colliding(&c3));
    }

    #[test]
    fn test_physics_update() {
        let mut body = PhysicsBody::new(Vector2::zero(), 1.0);
        body.apply_force(Vector2::new(10.0, 0.0));
        
        // After 1 second: accel = 10, vel = 10, pos = 10
        body.update(1.0);
        
        assert_eq!(body.position.x, 10.0);
        assert_eq!(body.velocity.x, 10.0);
        
        // Apply force again and update for another second
        body.apply_force(Vector2::new(10.0, 0.0));
        body.update(1.0);
        
        // vel was 10, accel is 10, so new vel = 20. pos was 10, new pos = 10 + 20 = 30
        assert_eq!(body.velocity.x, 20.0);
        assert_eq!(body.position.x, 30.0);
    }
}
pub fn physics_register(engine: &mut wolflang::WolfEngine) {
    engine.push_fn("check_collision", |args| {
        let x1 = match args.get(0) { Some(wolflang::tokens::Token::Float(f)) => *f as f32, Some(wolflang::tokens::Token::Integer(i)) => *i as f32, _ => return wolflang::tokens::Token::Boolean(false) };
        let y1 = match args.get(1) { Some(wolflang::tokens::Token::Float(f)) => *f as f32, Some(wolflang::tokens::Token::Integer(i)) => *i as f32, _ => return wolflang::tokens::Token::Boolean(false) };
        let w1 = match args.get(2) { Some(wolflang::tokens::Token::Float(f)) => *f as f32, Some(wolflang::tokens::Token::Integer(i)) => *i as f32, _ => return wolflang::tokens::Token::Boolean(false) };
        let h1 = match args.get(3) { Some(wolflang::tokens::Token::Float(f)) => *f as f32, Some(wolflang::tokens::Token::Integer(i)) => *i as f32, _ => return wolflang::tokens::Token::Boolean(false) };
        
        let x2 = match args.get(4) { Some(wolflang::tokens::Token::Float(f)) => *f as f32, Some(wolflang::tokens::Token::Integer(i)) => *i as f32, _ => return wolflang::tokens::Token::Boolean(false) };
        let y2 = match args.get(5) { Some(wolflang::tokens::Token::Float(f)) => *f as f32, Some(wolflang::tokens::Token::Integer(i)) => *i as f32, _ => return wolflang::tokens::Token::Boolean(false) };
        let w2 = match args.get(6) { Some(wolflang::tokens::Token::Float(f)) => *f as f32, Some(wolflang::tokens::Token::Integer(i)) => *i as f32, _ => return wolflang::tokens::Token::Boolean(false) };
        let h2 = match args.get(7) { Some(wolflang::tokens::Token::Float(f)) => *f as f32, Some(wolflang::tokens::Token::Integer(i)) => *i as f32, _ => return wolflang::tokens::Token::Boolean(false) };

        let c1 = Collider::new(x1, y1, w1, h1);
        let c2 = Collider::new(x2, y2, w2, h2);
        
        wolflang::tokens::Token::Boolean(c1.is_colliding(&c2))
    });
}
