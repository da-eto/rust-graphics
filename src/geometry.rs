use std::io;
use std::ops::{Add, Sub};
use std::path::Path;

#[derive(Debug, Copy, Clone)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T
}

impl<T> Add<Vec2D<T>> for Vec2D<T> where T: Add<T, Output = T> {
    type Output = Vec2D<T>;

    fn add(self, other: Vec2D<T>) -> Vec2D<T> {
        Vec2D { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<T> Sub<Vec2D<T>> for Vec2D<T> where T: Sub<T, Output = T> {
    type Output = Vec2D<T>;

    fn sub(self, other: Vec2D<T>) -> Vec2D<T> {
        Vec2D { x: self.x - other.x, y: self.y - other.y }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Vec3D<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Add<Vec3D<T>> for Vec3D<T> where T: Add<T, Output = T> {
    type Output = Vec3D<T>;

    fn add(self, other: Vec3D<T>) -> Vec3D<T> {
        Vec3D { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<T> Sub<Vec3D<T>> for Vec3D<T> where T: Sub<T, Output = T> {
    type Output = Vec3D<T>;

    fn sub(self, other: Vec3D<T>) -> Vec3D<T> {
        Vec3D { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

pub type Vec3Df = Vec3D<f64>;

#[derive(Debug)]
pub struct Model {
    pub verts: Vec<Vec3Df>,
    pub faces: Vec<Vec<u32>>
}

pub fn load_model_obj<P>(path: P) -> Result<Model, io::Error> where P: AsRef<Path> {
    Ok(Model { verts: Vec::new(), faces: Vec::new() })
}