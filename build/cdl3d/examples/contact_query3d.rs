extern crate nalgebra as na;

use cdl3d::query;
use cdl3d::shape::{Ball, Cuboid};
use na::{Isometry3, Vector3};

fn main() {
    let cuboid = Cuboid::new(Vector3::new(1.0, 1.0, 1.0));
    let ball = Ball::new(1.0);
    let prediction = 1.0;

    let cuboid_pos = Isometry3::identity();
    let ball_pos_penetrating = Isometry3::translation(1.0, 1.0, 1.0);
    let ball_pos_in_prediction = Isometry3::translation(2.0, 2.0, 2.0);
    let ball_pos_too_far = Isometry3::translation(3.0, 3.0, 3.0);

    let ctct_penetrating = query::contact(
        &ball_pos_penetrating.inv_mul(&cuboid_pos),
        &ball,
        &cuboid,
        prediction,
    )
    .unwrap();
    let ctct_in_prediction = query::contact(
        &ball_pos_in_prediction.inv_mul(&cuboid_pos),
        &ball,
        &cuboid,
        prediction,
    )
    .unwrap();
    let ctct_too_far = query::contact(
        &ball_pos_too_far.inv_mul(&cuboid_pos),
        &ball,
        &cuboid,
        prediction,
    )
    .unwrap();

    assert!(ctct_penetrating.unwrap().dist <= 0.0);
    assert!(ctct_in_prediction.unwrap().dist >= 0.0);
    assert_eq!(ctct_too_far, None);
}
