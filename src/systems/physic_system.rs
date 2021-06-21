use nalgebra::Vector2;
use rapier2d::prelude::*;

pub const DT: f32 = 1.0 / 60.0;

pub struct PhysicsSystem {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub narrow_phase: NarrowPhase,
    pub island_manager: IslandManager,
    pub joint_set: JointSet,
    pipeline: PhysicsPipeline,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhase,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
}

impl PhysicsSystem {
    pub fn new() -> Self {
        PhysicsSystem {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            pipeline: PhysicsPipeline::new(),
            gravity: vector![0.0, 0.0],
            integration_parameters: IntegrationParameters {
                dt: DT,
                ..Default::default()
            },
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            joint_set: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
        }
    }

    pub fn update(&mut self) {
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.joint_set,
            &mut self.ccd_solver,
            &self.physics_hooks,
            &self.event_handler,
        );
    }
}
