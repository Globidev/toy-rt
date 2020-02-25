use crate::prelude::*;
use crate::future::PyFuture;
use super::{
    float::FloatLike,
    material::{MaterialError, PyMaterial},
    vec3::PyVec3,
};

use trt_core::{
    hit::{RectBuilder, Sphere, HitBox, BVHNode},
    prelude::*,
};

use futures::prelude::*;

pub type SharedHit = PyFuture<Result<Rc<dyn Hit>, Rc<MaterialError>>>;

trt_py_class! { "Shape", PyShape,
    pub struct PyShape(SharedHit);
}

impl PyShape {
    pub fn shared_hit(&self) -> SharedHit {
        self.0.clone()
    }

    fn map<F, H>(&self, f: F) -> Self
    where
        F: FnOnce(Rc<dyn Hit>) -> H + 'static,
        H: Hit + 'static,
    {
        let mapped = self.shared_hit()
            .map(move |hit_res| {
                let hit = f(hit_res?);
                Ok(Rc::new(hit) as _)
            });

        Self(mapped)
    }
}

#[rpy::pyimpl]
impl PyShape {
    #[pyclassmethod]
    fn sphere(_cls: PyClassRef, center: PyVec3, radius: f32, material: PyMaterial) -> Self {
        let shared_hit = material
            .map_to_hit(move |mat| {
                Sphere::builder()
                    .radius(radius)
                    .center(center.into_vec())
                    .material(mat)
            });

        Self(shared_hit)
    }

    #[pyclassmethod]
    fn xy_rect(_cls: PyClassRef, x: (f32, f32), y: (f32, f32), z: f32, material: PyMaterial) -> Self {
        let shared_hit = material
            .map_to_hit(move |mat| {
                RectBuilder
                    .x(x.0..=x.1)
                    .y(y.0..=y.1)
                    .z(z)
                    .material(mat)
            });

            Self(shared_hit)
    }

    #[pyclassmethod]
    fn xz_rect(_cls: PyClassRef, x: (f32, f32), z: (f32, f32), y: f32, material: PyMaterial) -> Self {
        let shared_hit = material
            .map_to_hit(move |mat| {
                RectBuilder
                    .x(x.0..=x.1)
                    .z(z.0..=z.1)
                    .y(y)
                    .material(mat)
            });

            Self(shared_hit)
    }

    #[pyclassmethod]
    fn yz_rect(_cls: PyClassRef, y: (f32, f32), z: (f32, f32), x: f32, material: PyMaterial) -> Self {
        let shared_hit = material
            .map_to_hit(move |mat| {
                RectBuilder
                    .y(y.0..=y.1)
                    .z(z.0..=z.1)
                    .x(x)
                    .material(mat)
            });

            Self(shared_hit)
    }

    #[pyclassmethod]
    fn hitbox(_cls: PyClassRef, min: PyVec3, max: PyVec3, material: PyMaterial) -> Self {
        let shared_hit = material
            .map_to_hit(move |mat| {
                HitBox::new(
                    min.into_vec(),
                    max.into_vec(),
                    mat
                )
            });

        Self(shared_hit)
    }

    #[pyclassmethod]
    fn bvh_node(_cls: PyClassRef, objects: PyListRef, vm: &VirtualMachine) -> PyResult<Self> {
        let world_futures: Vec<_> = objects
            .borrow_elements()
            .iter()
            .map(|py_obj| {
                let shape: PyRef<PyShape> = py_obj.clone().try_into_ref(vm)?;
                Ok(shape.shared_hit().shared())
            })
            .collect::<PyResult<_>>()?;

        let node_future = future::try_join_all(world_futures)
            .map_ok(|mut world| {
                let node = BVHNode::new(&mut world, 0., 1.);
                Rc::new(node) as _
            });

        Ok(Self(PyFuture::new(node_future)))
    }

    #[pymethod]
    fn flip_normals(&self) -> Self {
        self.map(|h| h.flip_normals())
    }

    #[pymethod]
    fn rotate_x(&self, angle: FloatLike) -> Self {
        self.map(move |h| h.rotate_x(angle.as_f32()))
    }

    #[pymethod]
    fn rotate_y(&self, angle: FloatLike) -> Self {
        self.map(move |h| h.rotate_y(angle.as_f32()))
    }

    #[pymethod]
    fn rotate_z(&self, angle: FloatLike) -> Self {
        self.map(move |h| h.rotate_z(angle.as_f32()))
    }

    #[pymethod]
    fn translate(&self, offset: PyVec3) -> Self {
        self.map(move |h| h.translate(offset.into_vec()))
    }

    #[pymethod]
    fn constant_medium(&self, density: FloatLike, color: PyVec3) -> Self {
        self.map(move |h| h.constant_medium(density.as_f32(), color.into_vec()))
    }
}
