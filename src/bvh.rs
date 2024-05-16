use std::cmp::Ordering;

use crate::aabb::Aabb;
use crate::hittable::{HitData, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::Dim;

pub struct Bvh<'a> {
    pub left: &'a Box<dyn Hittable>,
    pub right: &'a Box<dyn Hittable>,
    pub bbox: Aabb,
}

impl Bvh<'_> {
    // make sure start, end < len(objects) - 1 and start < end
    pub fn new(objects: &mut Vec<Box<dyn Hittable>>, start: usize, end: usize) -> Self {
        // Build bounding box for span of objects included in this BVH node
        let mut bbox = Aabb::empty();
        for object in objects[start..end].iter() {
            bbox = Aabb::new_from_boxes(bbox, object.bounding_box())
        }

        let longest_axis = bbox.longest_axis();

        let object_span = end - start;
        let (left, right): (&Box<dyn Hittable>, &Box<dyn Hittable>) = match object_span {
            1 => (&objects[start], &objects[start]),
            2 => (&objects[start], &objects[start + 1]),
            _ => {
                let random_dim: Dim = rand::random();
                objects.sort_by(move |&a, &b| -> Ordering {
                    match Self::box_compare(a, b, longest_axis) {
                        true => Ordering::Less,
                        false => Ordering::Greater,
                    }
                });

                let mid = start + object_span / 2;
                (
                    &(Box::new(Self::new(objects, start, mid)) as Box<dyn Hittable>),
                    &(Box::new(Self::new(objects, mid, end)) as Box<dyn Hittable>),
                )
            }
        };
        Self { left, right, bbox }
    }

    pub fn new_from_list() {}

    fn box_compare(a: Box<dyn Hittable>, b: Box<dyn Hittable>, dim: Dim) -> bool {
        let a_interval = a.bounding_box().get(dim);
        let b_interval = b.bounding_box().get(dim);
        a_interval.min < b_interval.min
    }
}

impl Hittable for Bvh<'_> {
    fn hit(&self, ray: Ray, interval: Interval, hit_data: &mut HitData) -> bool {
        if !self.bbox.hit(ray, interval) {
            return false;
        }

        let left_hit = self.left.hit(ray, interval, hit_data);
        let right_hit = self.right.hit(ray, interval, hit_data);

        left_hit || right_hit
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
