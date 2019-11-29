use crate::prelude::{Hit, ParallelHit, AABB, HitRecord, Ray};

use crate::utils::{thread_rng, SliceRandom};
use std::sync::Arc;
use std::cmp::Ordering;

pub struct BVHNode {
    left: Arc<dyn ParallelHit>,
    right: Arc<dyn ParallelHit>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(hittables: &mut [Arc<dyn ParallelHit>], time0: f32, time1: f32) -> Self {
        let compare = [box_x_cmp, box_y_cmp, box_z_cmp].choose(&mut thread_rng()).unwrap();

        hittables.sort_by(|a, b| compare(a.as_ref(), b.as_ref()));

        let (left, right) = match hittables.len() {
            1 => (hittables[0].clone(), hittables[0].clone()),
            2 => (hittables[0].clone(), hittables[1].clone()),
            n => {
                let (left_l, right_l) = hittables.split_at_mut(n / 2);
                (
                    Arc::new(BVHNode::new(left_l, time0, time1)) as _,
                    Arc::new(BVHNode::new(right_l, time0, time1)) as _,
                )
            }
        };

        let box_left = left.bounding_box(time0, time1).expect("missing bbox in BVH::new");
        let box_right = right.bounding_box(time0, time1).expect("missing bbox in BVH::new");

        Self {
            left,
            right,
            bbox: AABB::surrounding_box(box_left, box_right),
        }
    }
}

impl Hit for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        if self.bbox.hit(ray, t_min, t_max) {
            let hit_left = self.left.hit(ray, t_min, t_max);
            let hit_right = self.right.hit(ray, t_min, t_max);

            match (hit_left, hit_right) {
                (Some(left_rec), Some(right_rec)) => {
                    if left_rec.t < right_rec.t { Some(left_rec) } else { Some(right_rec) }
                },
                (Some(left_rec), None) => Some(left_rec),
                (None, Some(right_rec)) => Some(right_rec),
                _ => None,
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}

fn box_x_cmp(ah: &dyn ParallelHit, bh: &dyn ParallelHit) -> Ordering {
    let box_left = ah.bounding_box(0., 0.).expect("missing bbox in BVH::new");
    let box_right = bh.bounding_box(0., 0.).expect("missing bbox in BVH::new");

    PartialOrd::partial_cmp(&box_left.min.x(), &box_right.min.x())
        .expect("got NaNs")
}

fn box_y_cmp(ah: &dyn ParallelHit, bh: &dyn ParallelHit) -> Ordering {
    let box_left = ah.bounding_box(0., 0.).expect("missing bbox in BVH::new");
    let box_right = bh.bounding_box(0., 0.).expect("missing bbox in BVH::new");

    PartialOrd::partial_cmp(&box_left.min.y(), &box_right.min.y())
        .expect("got NaNs")
}

fn box_z_cmp(ah: &dyn ParallelHit, bh: &dyn ParallelHit) -> Ordering {
    let box_left = ah.bounding_box(0., 0.).expect("missing bbox in BVH::new");
    let box_right = bh.bounding_box(0., 0.).expect("missing bbox in BVH::new");

    PartialOrd::partial_cmp(&box_left.min.z(), &box_right.min.z())
        .expect("got NaNs")
}
