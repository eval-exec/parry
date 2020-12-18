extern crate nalgebra as na;

use cdl3d::math::Real;
use cdl3d::query;
use cdl3d::shape::{Ball, Cuboid};
use na::{Isometry3, Vector3};

fn main() {
    let cuboid = Cuboid::new(Vector3::new(1.0, 1.0, 1.0));
    let ball = Ball::new(1.0);

    let cuboid_pos = Isometry3::identity();
    let ball_pos_intersecting = Isometry3::translation(1.0, 1.0, 1.0);
    let ball_pos_will_touch = Isometry3::translation(2.0, 2.0, 2.0);
    let ball_pos_wont_touch = Isometry3::translation(3.0, 3.0, 3.0);

    let box_vel1 = Vector3::new(-1.0, 1.0, 1.0);
    let box_vel2 = Vector3::new(1.0, 1.0, 1.0);

    let ball_vel1 = Vector3::new(2.0, 2.0, 2.0);
    let ball_vel2 = Vector3::new(-0.5, -0.5, -0.5);

    let toi_intersecting = query::time_of_impact(
        &ball_pos_intersecting.inv_mul(&cuboid_pos),
        &(box_vel1 - ball_vel1),
        &ball,
        &cuboid,
        Real::MAX,
        0.0,
    )
    .unwrap();
    let toi_will_touch = query::time_of_impact(
        &ball_pos_will_touch.inv_mul(&cuboid_pos),
        &(box_vel2 - ball_vel2),
        &ball,
        &cuboid,
        Real::MAX,
        0.0,
    )
    .unwrap();
    let toi_wont_touch = query::time_of_impact(
        &ball_pos_wont_touch.inv_mul(&cuboid_pos),
        &(box_vel1 - ball_vel1),
        &ball,
        &cuboid,
        Real::MAX,
        0.0,
    )
    .unwrap();

    assert_eq!(toi_intersecting.map(|toi| toi.toi), Some(0.0));
    assert!(toi_will_touch.is_some() && toi_will_touch.unwrap().toi > 0.0);
    assert_eq!(toi_wont_touch.map(|toi| toi.toi), None);
}
