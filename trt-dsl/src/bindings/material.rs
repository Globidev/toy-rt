use crate::{future::PyFuture, prelude::*};
use super::{shape::SharedHit, vec3::PyVec3};

use trt_core::{
    material::{Dielectric, Diffuse, Lambertian, Metal},
    prelude::*,
    texture::{Checker, Image, Constant},
};

use rpy::obj::objstr::PyStringRef;

use std::fmt;

#[derive(Debug)]
pub enum MaterialError {
    ImageFetch { err: reqwest::Error, url: String },
    ImageLoad { err: image::ImageError, url: String },
}

impl fmt::Display for MaterialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaterialError::ImageFetch { err, url } => write!(f, "Error fetching \"{}\": {}", url, err),
            MaterialError::ImageLoad { err, url } => write!(f, "Unsupported image format for: \"{}\": {}", url, err)
        }
    }
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
    fn checker(_cls: PyClassRef, col1: PyVec3, col2: PyVec3, repeat_frequency: f32) -> Self {
        let tx1 = Constant::new(col1.into_vec());
        let tx2 = Constant::new(col2.into_vec());
        let checker = Checker::new(tx1, tx2, repeat_frequency);

        Self::new(Lambertian::new(checker))
    }

    #[pyclassmethod]
    fn image(_cls: PyClassRef, url: PyStringRef) -> Self {
        Self(PyFuture::new(async move {
            let url = url.as_str();

            let resp = reqwest::get(url)
                .await
                .map_err(|err| Rc::new(MaterialError::ImageFetch { err, url: url.to_owned() }))?;

            let bytes = resp
                .bytes()
                .await
                .map_err(|err| Rc::new(MaterialError::ImageFetch { err, url: url.to_owned() }))?;

            let raw_img = image::load_from_memory(&bytes)
                .map_err(|err| Rc::new(MaterialError::ImageLoad { err, url: url.to_owned() }))?
                .into_rgb();

            let (width, height) = raw_img.dimensions();
            let img = Image::load(raw_img.into_vec(), width as _, height as _);

            Ok(Rc::new(Lambertian::new(img)) as _)
        }))
    }
}
