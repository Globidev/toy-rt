use crate::{future::PyFuture, prelude::*};
use super::{shape::SharedHit, vec3::PyVec3};

use trt_core::{
    material::{Dielectric, Diffuse, Lambertian, Metal},
    prelude::*,
    texture::Image,
};

use rpy::obj::objstr::PyStringRef;

#[derive(Debug)]
pub enum MaterialError {
    ImageFetch(reqwest::Error),
    ImageLoad(image::ImageError),
}

type MaterialResult = Result<Rc<dyn Material>, Rc<MaterialError>>;

trt_py_class! { "Material", PyMaterial,
    #[derive(Clone)]
    pub struct PyMaterial(PyFuture<MaterialResult>);
}

impl PyMaterial {
    pub fn new<Mat: Material + 'static>(mat: Mat) -> Self {
        Self(PyFuture::ready(Ok(Rc::new(mat))))
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

impl TryFromObject for PyMaterial {
    fn try_from_object(vm: &VirtualMachine, obj: PyObjectRef) -> PyResult<Self> {
        let mat: PyRef<Self> = obj.try_into_ref(vm)?;

        Ok((*mat).clone())
    }
}

#[rpy::pyimpl]
impl PyMaterial {
    #[pyclassmethod]
    fn dielectric(_cls: PyClassRef, ref_idx: f32) -> Self {
        Self::new(Dielectric::new(ref_idx))
    }

    #[pyclassmethod]
    fn diffuse_color(_cls: PyClassRef, color: PyVec3) -> Self {
        Self::new(Diffuse::colored(color.into_vec()))
    }

    #[pyclassmethod]
    fn matte(_cls: PyClassRef, color: PyVec3) -> Self {
        Self::new(Lambertian::colored(color.into_vec()))
    }

    #[pyclassmethod]
    fn metallic_fuzzed(_cls: PyClassRef, albedo: PyVec3, fuzz: f32) -> Self {
        Self::new(Metal::new(albedo.into_vec(), fuzz))
    }

    #[pyclassmethod]
    fn image(_cls: PyClassRef, url: PyStringRef) -> Self {
        Self(PyFuture::new(async move {
            let resp = reqwest::get(url.as_str())
                .await
                .map_err(|e| Rc::new(MaterialError::ImageFetch(e)))?;

            let bytes = resp
                .bytes()
                .await
                .map_err(|e| Rc::new(MaterialError::ImageFetch(e)))?;

            let raw_img = image::load_from_memory(&bytes)
                .map_err(|e| Rc::new(MaterialError::ImageLoad(e)))?
                .into_rgb();

            let (width, height) = raw_img.dimensions();
            let img = Image::load(raw_img.into_vec(), width as _, height as _);

            Ok(Rc::new(Lambertian::new(img)) as _)
        }))
    }
}
