use crate::systems::player_system::player_mesh_creator::build_player_collider;
use nalgebra::Vector2;
use rapier2d::dynamics::RigidBodyHandle;
use rapier2d::prelude::*;

use crate::constants::*;
use crate::PhysicsSystem;

#[derive(PartialEq)]
enum PlayerState {
    Sliding,
    Boosting,
}

pub struct Player {
    // pub collider_handles: Vec<ColliderHandle>,
    pub body_handle: RigidBodyHandle,
    pub ang_acc: f32,
    pub lin_acc: f32,
    state: PlayerState,
}

impl Player {
    pub fn new(physic_system: &mut PhysicsSystem) -> Self {
        let rb = RigidBodyBuilder::new_dynamic()
            .translation(Vector2::new(MIDDLE.0, MIDDLE.1 - 300.0))
            .linear_damping(0.5)
            .angular_damping(10.0)
            .can_sleep(false)
            .ccd_enabled(true)
            .build();
        let body_handle = physic_system.rigid_body_set.insert(rb);
        let mut collider_handles: Vec<ColliderHandle> = Vec::new();
        let mut colliders = build_player_collider();
        for _i in 0..colliders.len() {
            collider_handles.push(physic_system.collider_set.insert_with_parent(
                colliders.pop().unwrap(),
                body_handle,
                &mut physic_system.rigid_body_set,
            ));
        }

        Player {
            state: PlayerState::Sliding,
            ang_acc: 0.0,
            lin_acc: 0.0,
            body_handle,
            // collider_handles: collider_handles,
        }
    }

    pub fn rotate(&mut self, value: f32) {
        self.ang_acc = value;
    }

    pub fn boost(&mut self, value: f32) {
        self.lin_acc = value;
        if value < 0.0 {
            self.state = PlayerState::Boosting;
        } else {
            self.state = PlayerState::Sliding;
        }
    }
}
