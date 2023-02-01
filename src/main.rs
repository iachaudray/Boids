use bevy::prelude::*;
use rand::prelude::*;


fn main() {

    App::new().add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_boids)
        .add_system(boid_movement)
        .add_system(bevy::window::close_on_esc)
        .run();

}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_boids(mut commands: Commands) {

    for _i in 1..520 {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(5.0, 5.0, 5.0),
                translation: Vec3::new(thread_rng().gen_range(-640..640) as f32, thread_rng().gen_range(-360..360) as f32, 0.0),
                ..default()
            },
            ..default()
        }).insert(Boid {
            num_neighbors: 1,
            velocity: Vec2::new(thread_rng().gen_range(-2.0..2.0), thread_rng().gen_range(-2.0..2.0)),
            avg_vel: Vec2::ZERO,
            close_dx: Vec2::ZERO,
            center_avg: Vec2::ZERO
        });
    }
}


#[derive(Component, Default)]
struct Boid {
    pub num_neighbors: i8,
    pub velocity: Vec2,
    pub avg_vel: Vec2,
    pub close_dx: Vec2,
    pub center_avg: Vec2,
}





fn boid_movement(mut boids: Query<(&mut Boid, &mut Transform)>, time: Res<Time>) {
    let mut iter = boids.iter_combinations_mut();

    while let Some([(mut boid1, transform1), (mut boid2, transform2)]) =
        iter.fetch_next()
    {
        let distance = transform1.translation.distance(transform2.translation);
        if distance < 30.0 {
            boid1.close_dx.x += transform1.translation.x - transform2.translation.x;
            boid1.close_dx.y += transform1.translation.y - transform2.translation.y;

            boid2.close_dx.x += transform2.translation.x - transform1.translation.x;
            boid2.close_dx.y += transform2.translation.y - transform1.translation.y;
        } else if distance < 150.0 {
            boid1.avg_vel.x += boid2.velocity.x;
            boid1.avg_vel.y += boid2.velocity.y;
            boid1.num_neighbors += 1;
            boid1.center_avg.x += transform2.translation.x;
            boid1.center_avg.y += transform2.translation.y;
            
            boid2.avg_vel.x += boid1.velocity.x;
            boid2.avg_vel.y += boid1.velocity.y;
            boid2.num_neighbors += 1;
            boid2.center_avg.x += transform1.translation.x;
            boid2.center_avg.y += transform1.translation.y;

        }
       


    }
    for (mut boid, mut transform) in boids.iter_mut() {
        let mut avg_vel_added_x: f32 = 0.0;
        let mut avg_vel_added_y: f32 = 0.0;
        
        let has_neighbors = boid.num_neighbors > 1;
        if has_neighbors {
            let center_x = boid.center_avg.x / (boid.num_neighbors as f32);
            let center_y = boid.center_avg.y / (boid.num_neighbors as f32);
            avg_vel_added_x = boid.avg_vel.x / (boid.num_neighbors as f32);
            avg_vel_added_y = boid.avg_vel.y / (boid.num_neighbors as f32);
            boid.velocity.x -= (transform.translation.x - center_x) * 0.001;
            boid.velocity.y -= (transform.translation.y - center_y) * 0.001;
        }
        
        boid.velocity.x += (boid.close_dx.x * 0.005) + (avg_vel_added_x*0.01);
        boid.velocity.y += (boid.close_dx.y * 0.005) + (avg_vel_added_y*0.01);
        
        

        if boid.velocity.length() > 5.0 {
            let normalized_vel = Vec2::from(boid.velocity.normalize() * 5.0);
            boid.velocity = normalized_vel;
        }
        transform.translation.x += boid.velocity.x * time.delta_seconds() * 45.0;
        transform.translation.y += boid.velocity.y * time.delta_seconds() * 45.0;
        if has_neighbors {
            boid.num_neighbors = 0;
            boid.close_dx = Vec2::ZERO;
            boid.avg_vel = Vec2::ZERO;
            boid.center_avg = Vec2::ZERO;
        }
        



        if transform.translation.x > 640.0 {
            transform.translation.x = -640.0;
        } else if transform.translation.x < -640.0 {
            transform.translation.x = 640.0;
        }
        if transform.translation.y > 360.0 {
            transform.translation.y = -360.0;
        } else if transform.translation.y < -360.0 {
            transform.translation.y = 360.0;
        }


    }
}














