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

impl<'b> Bvh<'b> {
    // make sure start, end < len(objects) - 1 and start < end
    pub fn new(mut objects: Vec<&'b Box<dyn Hittable>>) -> Self {
        // Build bounding box for span of objects included in this BVH node
        let mut bbox = Aabb::empty();
        for &object in objects.iter() {
            bbox = Aabb::new_from_boxes(bbox, object.bounding_box())
        }

        let longest_axis = bbox.longest_axis();

        match objects.len() {
            1 => Self {
                left: objects[0],
                right: objects[0],
                bbox,
            },
            2 => Self {
                left: objects[0],
                right: objects[1],
                bbox,
            },
            _ => {
                objects.sort_by(move |&a, &b| -> Ordering {
                    match Self::box_compare(a, b, longest_axis) {
                        true => Ordering::Less,
                        false => Ordering::Greater,
                    }
                });

                let mid = objects.len() / 2;
                let left_objects = objects[..mid].to_vec();
                let right_objects = objects[mid..].to_vec();
                Self {
                    left: &(Box::new(Self::new(left_objects)) as Box<dyn Hittable>),
                    right: &(Box::new(Self::new(right_objects.clone())) as Box<dyn Hittable>),
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

impl<'a> Hittable for Bvh<'a> {
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
