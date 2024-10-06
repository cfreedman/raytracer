use std::cmp::Ordering;

use crate::aabb::Aabb;
use crate::hittable::{HitData, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Dim;

pub struct Bvh {
    pub left: Box<dyn Hittable>,
    pub right: Option<Box<dyn Hittable>>,
    pub bbox: Aabb,
}

impl Bvh {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> Self {
        // Build bounding box for span of objects included in this BVH node
        let bbox = objects.iter().fold(Aabb::empty(), |acc, object| Aabb::new_from_boxes(acc, object.bounding_box()));

        let longest_axis = bbox.longest_axis();

        match objects.len() {
            1 => Self {
                left: objects.remove(0),
                right: None,
                bbox,
            },
            2 => Self {
                left: objects.remove(0),
                right: Some(objects.remove(0)),
                bbox,
            },
            _ => {
                objects.sort_by(move |a, b| -> Ordering {
                    match Self::box_compare(&a, &b, longest_axis) {
                        true => Ordering::Less,
                        false => Ordering::Greater,
                    }
                });

                let mid = objects.len() / 2;
                let left_objects = objects.split_off(mid);
                let right_objects = objects;
                Self {
                    left: Box::new(Self::new(left_objects)),
                    right: Some(Box::new(Self::new(right_objects))),
                    bbox,
                }
            }
        }
    }

    pub fn new_from_list() {}

    fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, dim: Dim) -> bool {
        let a_interval = a.bounding_box().get(dim);
        let b_interval = b.bounding_box().get(dim);
        a_interval.min < b_interval.min
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: Ray, interval: Interval, hit_data: &mut HitData) -> bool {
        if !self.bbox.hit(ray, interval) {
            return false;
        }

        let left_hit = self.left.hit(ray, interval, hit_data);
        let right_hit = match &self.right {
            Some(obj) => obj.hit(ray, interval, hit_data),
            None => false
        };

        left_hit || right_hit
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
