// PLAYER
pub const PLAYER_ACC: f32 = 80000.0;
pub const PLAYER_TURN: f32 = 600000.0;

// World
pub const MIDDLE: (f32, f32) = (400.0, 300.0);
pub const EXTERIOR_RADIUS: f32 = 600.0;
pub const EXTERIOR_PULL_FORCE: f32 = 80000.0;
pub const EARTH_RADIUS: f32 = 100.0;
pub const SCANLINE_FULL_ROTATION_TIME: f32 = 5.0;

// ENEMIES
pub const EARTH_GRAVITY: f32 = 9.807 * 8.0;
pub const ENEMY_MAX_SIZE: f32 = 20.0;
pub const ENEMY_MIN_SIZE: f32 = 10.0;
pub const ENEMY_MAX_LIFETIME: f32 = 1.0;
pub const ENEMY_BURNUP_DISTANCE: f32 = EARTH_RADIUS * 1.8;
pub const OUTER_SPACE_DISTANCE: f32 = 4000.0;

// pub const SPAWN_RANGE: (f32, f32) = (EXTERIOR_RADIUS, EXTERIOR_RADIUS + 200.0);
pub const SPAWN_RANGE: (f32, f32) = (EARTH_RADIUS + 30.0, EARTH_RADIUS + 200.0);
pub const SPAWN_TIME: f32 = 2.772;

// DEPTHS
pub const _D_FOREGROUND: i8 = -10;
pub const D_ANIMATED: i8 = 0;
pub const D_EARTH: i8 = 5;
pub const D_SCANLINE: i8 = 10;
pub const _D_NEBULA: i8 = 20;
pub const _D_STARS: i8 = 30;

// COLLIDER_TYPES
pub const COLL_PLAYER: u128 = 1;
pub const COLL_PLAYER_HEAVY: u128 = 2;
pub const COLL_SMALL: u128 = 30;
pub const COLL_MIDDLE: u128 = 31;
pub const COLL_BIG: u128 = 32;

// GAME_VARS
pub const HEALTH: i32 = 1000;
pub const DAMAGE_SMALL: i32 = 0;
pub const DAMAGE_MIDDLE: i32 = 50;
pub const DAMAGE_BIG: i32 = 100;
pub const DAMAGE_PLAYER: i32 = 50;
pub const DAMAGE_COOLDOWN: f32 = 0.5;
