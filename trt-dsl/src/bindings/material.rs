use trt_core::{
    prelude::*, material::{Dielectric, Lambertian, Diffuse, Metal}, texture::{Image, ImageLoadError},
};

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, PyValue},
};

use std::{fmt, rc::Rc};

use super::{vec3::PyVec3, shape::SharedHit};
use rpy::{obj::objstr::PyStringRef, pyobject::{TryFromObject, PyObjectRef, PyRef}};
use crate::future::PyFuture;

#[derive(Debug)]
pub enum MaterialError {
    ImageFetch(reqwest::Error),
    ImageLoad(ImageLoadError)
}

type SharedMaterial = PyFuture<Result<Rc<dyn Material>, Rc<MaterialError>>>;

#[rpy::pyclass(name = "Material")]
// #[derive(Debug)]
pub struct PyMaterial(SharedMaterial);

impl fmt::Debug for PyMaterial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Material>")
    }
}

impl PyMaterial {
    pub fn new<Mat: Material + 'static>(mat: Mat) -> Self {
        Self(PyFuture::ready(Ok(Rc::new(mat))))
    }

    pub fn shared_material(&self) -> SharedMaterial {
        self.0.clone()
    }

    pub fn map_to_hit<F, H>(self, f: F) -> SharedHit
    where
        F: FnOnce(Rc<dyn Material>) -> H + 'static,
        H: Hit + 'static,
    {
        self.0.map(move |mat_res| {
            let hit = f(mat_res?);
            Ok(Rc::new(hit) as _)
        })
    }
}

impl PyValue for PyMaterial {
    fn class(vm: &rpy::VirtualMachine) -> PyClassRef {
        vm.class(super::TRT_MODULE_NAME, "Material")
    }
}

impl TryFromObject for PyMaterial {
    fn try_from_object(vm: &rpy::VirtualMachine, obj: PyObjectRef) -> PyResult<Self> {
        let x: PyRef<Self> = obj.downcast()
            .map_err(|e| vm.new_type_error(format!("Expected Material, got: {}", e)))?;
        Ok(Self(x.shared_material().clone()))
    }
}

#[rpy::pyimpl]
impl PyMaterial {
    #[pyclassmethod]
    fn dielectric(_cls: PyClassRef, ref_idx: f32, _vm: &rpy::VirtualMachine) -> Self {
        Self::new(Dielectric::new(ref_idx))
    }

    #[pyclassmethod]
    fn diffuse_color(_cls: PyClassRef, color: PyVec3, _vm: &rpy::VirtualMachine) -> Self {
        Self::new(Diffuse::colored(color.into_vec()))
    }

    #[pyclassmethod]
    fn matte(_cls: PyClassRef, color: PyVec3, _vm: &rpy::VirtualMachine) -> Self {
        Self::new(Lambertian::colored(color.into_vec()))
    }

    #[pyclassmethod]
    fn metallic(_cls: PyClassRef, albedo: PyVec3, _vm: &rpy::VirtualMachine) -> Self {
        Self::new(Metal::new(albedo.into_vec(), 0.))
    }

    #[pyclassmethod]
    fn metallic_fuzzed(_cls: PyClassRef, albedo: PyVec3, fuzz: f32, _vm: &rpy::VirtualMachine) -> Self {
        Self::new(Metal::new(albedo.into_vec(), fuzz))
    }

    #[pyclassmethod]
    fn image(_cls: PyClassRef, url: PyStringRef, _vm: &rpy::VirtualMachine) -> Self {
        Self(PyFuture::new(async move {
            let resp = reqwest::get(url.as_str()).await
                .map_err(|e| Rc::new(MaterialError::ImageFetch(e)))?;

            let bytes = resp.bytes().await
                .map_err(|e| Rc::new(MaterialError::ImageFetch(e)))?;

            let img = Image::load_from_memory(&bytes)
                .map_err(|e| Rc::new(MaterialError::ImageLoad(e)))?;

            Ok(Rc::new(Lambertian::new(img)) as _)
        }))
    }
}
