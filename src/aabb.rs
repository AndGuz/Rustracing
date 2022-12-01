pub struct aabb{
    minimum: Point3,
    maximum: Point3
}

impl aabb {
    pub fn new(a: Point3, b: Point3) -> aabb{
        aabb { minimum: a, maximum: b }
    }
    

    pub fn min(&self) -> Vec3 {
        self.minimum
    }

    pub fn max(&self) -> Vec3 {
        self.maximum
    }
}
