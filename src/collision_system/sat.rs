use ggez::nalgebra as na;
use na::Vector2;

use crate::collision_system::colliders::poly_collider::PolyCollider;
use crate::collision_system::PolyCollision;

pub struct Coll<'a> {
    pub first: &'a PolyCollider,
    pub second: &'a PolyCollider,
    pub offset: Vector2<f32>,
    pub vel_rel: Vector2<f32>,

    pub n: Vector2<f32>,
    pub t: f32,

    pub axis: Vec<Vector2<f32>>,
    pub taxis: Vec<f32>,
}

impl<'a> Coll<'a> {
    pub fn new(
        first: &'a PolyCollider,
        second: &'a PolyCollider,
        first_vel: &Vector2<f32>,
        second_vel: &Vector2<f32>,
    ) -> Self {
        let offset = first.center - second.center;
        let vel_rel = first_vel - second_vel;
        Coll {
            first,
            second,
            offset,
            vel_rel,
            n: Vector2::new(0.0, 0.0),
            t: 1.0,
            axis: Vec::new(),
            taxis: Vec::new(),
        }
    }

    pub fn collide(&mut self) -> bool {
        let mut i_num_axes: usize = 0;

        let f_vel2 = self.vel_rel.dot(&self.vel_rel);
        if f_vel2 > 0.000001 {
            self.axis.push(self.vel_rel.normalize());
            if !self.interval_intersect(&mut i_num_axes) {
                return false;
            }
            i_num_axes += 1;
        }

        //Test separation axes of A
        let edges = self.first.edges_of();
        let orthogonals: Vec<Vector2<f32>> = edges.iter().map(|edge| otho(edge)).collect();
        for ortho in orthogonals {
            self.axis.push(ortho);
            if !self.interval_intersect(&mut i_num_axes) {
                return false;
            };
            i_num_axes += 1;
        }

        //Test separation axes of second
        let edges = self.second.edges_of();
        let orthogonals: Vec<Vector2<f32>> = edges.iter().map(|edge| otho(edge)).collect();
        for ortho in orthogonals {
            self.axis.push(ortho);
            if !self.interval_intersect(&mut i_num_axes) {
                return false;
            };
            i_num_axes += 1;
        }
        //find the MDT among all the separation vectors
        if !self.find_collision_plane() {
            return false;
        }

        //makes sure the push vector is pushing A away from B
        if self.n.dot(&self.offset) < 0.0 {
            self.n.x = -self.n.x;
            self.n.y = -self.n.y;
        }
        // coll.n = vec2.mulMat(coll.n, coll.b.orientation);		//!!!!!!!!!!!

        return true;
    }

    fn interval_intersect(&mut self, i_num_axes: &mut usize) -> bool {
        let (mut min1, mut max1) = Coll::calculate_interval(self.first, self.axis[*i_num_axes]);
        let (min2, max2) = Coll::calculate_interval(self.second, self.axis[*i_num_axes]);

        // let h = self.offset.dot(&self.axis[*i_num_axes]);
        // min1 += h;
        // max1 += h;

        let d0 = min1 - max2;
        let d1 = min2 - max1;

        /*
            if max1 >= min2 && max2 >= min1 {
            let d = (max2 - min1).min(max1 - min2);
            let d_over_o_squared = d / o.dot(o) + 1e-10;
            let pv = d_over_o_squared * o;
            return (false, pv);
        }

        */

        if d0 > 0.0 || d1 > 0.0 {
            // var v = vec2.dot(coll.vel, coll.axis[iNumAxes]);
            // //small velocity, so only the overlap test will be relevant
            // if(Math.abs(v) < 0.0000001) return false;

            // var t0 = -d0 / v;	//time of impact to d0 reaches 0
            // var t1 = d1 / v;	//time of impact to d0 reaches 1

            // if (t0 > t1) {var temp = t0; t0 = t1; t1 = temp;}
            // coll.taxis[iNumAxes] = (t0 > 0.0) ? t0 : t1;

            // if(coll.taxis[iNumAxes] < 0.0 || coll.taxis[iNumAxes] > coll.t) return false;

            return false;
        } else {
            //overlap. get the interval, as a the biggest of |d0| and |d1|
            //return negative number to mark it as an overlap
            self.taxis.push(d0.max(d1));
            return true;
        }
    }

    fn find_collision_plane(&mut self) -> bool {
        let mut mini = false;
        for (i, axis) in self.axis.iter().enumerate() {
            let n = axis.norm();
            self.taxis[i] /= n + 1e-10;

            if self.taxis[i] > self.t || !mini {
                mini = true;
                self.t = self.taxis[i];
                self.n = axis.clone();
            }
        }
        return mini;
    }

    fn calculate_interval(collider: &PolyCollider, axis: Vector2<f32>) -> (f32, f32) {
        let vertices = collider.vertices();
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        for vertex in vertices {
            let projection = vertex.dot(&axis);
            min = min.min(projection);
            max = max.max(projection);
        }

        (min, max)
    }
}

pub fn is_collision(me: &impl PolyCollision, other: &impl PolyCollision) -> (bool, Vector2<f32>) {
    let mut edges = me.edges_of();
    edges.extend(other.edges_of().iter());
    let mut orthogonals: Vec<Vector2<f32>> = Vec::with_capacity(edges.len());
    for (_i, v) in edges.iter().enumerate() {
        orthogonals.push(otho(v));
    }
    let mut push_vectors: Vec<Vector2<f32>> = Vec::new();
    for o in &orthogonals {
        let (separated, pv) = is_separating_axis(o, me, other);
        if separated {
            return (false, Vector2::new(0.0, 0.0));
        } else {
            push_vectors.push(pv);
        }
    }

    let mpv = min_vec(&push_vectors);
    (true, mpv)
}

fn is_separating_axis(
    o: &Vector2<f32>,
    p1: &impl PolyCollision,
    p2: &impl PolyCollision,
) -> (bool, Vector2<f32>) {
    let p1_vertices = p1.vertices();
    let p2_vertices = p2.vertices();

    let mut min1 = f32::MAX;
    let mut min2 = f32::MAX;
    let mut max1 = -f32::MAX;
    let mut max2 = -f32::MAX;

    for v in &p1_vertices {
        let projection = v.dot(o);

        min1 = min1.min(projection);
        max1 = max1.max(projection);
    }

    for v in &p2_vertices {
        let projection = v.dot(o);

        min2 = min2.min(projection);
        max2 = max2.max(projection);
    }

    if max1 >= min2 && max2 >= min1 {
        let d = (max2 - min1).min(max1 - min2);
        let d_over_o_squared = d / o.dot(o) + 1e-10;
        let pv = d_over_o_squared * o;
        return (false, pv);
    }
    (true, Vector2::new(0.0, 0.0))
}

fn otho(v: &Vector2<f32>) -> Vector2<f32> {
    Vector2::new(-v.y, v.x)
}

fn min_vec(vec: &Vec<Vector2<f32>>) -> Vector2<f32> {
    let mut smallest = Vector2::new(f32::MAX, f32::MAX);
    let mut smallest_len = f32::MAX;
    for v in vec {
        let len = v.norm();
        if len < smallest_len {
            smallest_len = len;
            smallest = v.clone();
        }
    }
    smallest
}
