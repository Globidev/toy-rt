use crate::bindings::material::MaterialError;
use trt_core::{
    hit::HitList,
    prelude::*,
    scene::Scene,
};

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, TryIntoRef, TryFromObject},
};

use super::{camera::PyCamera, vec3::PyVec3, shape::PyShape};
use rpy::{obj::objlist::PyListRef, pyobject::{PyRef, PyObjectRef}};
use std::rc::Rc;

use crate::future::PyFuture;
use futures::prelude::*;

pub type DynScene = Scene<HitList<Rc<dyn Hit>>>;

trt_py_class! { "Scene", PyScene,
    pub struct PyScene(PyFuture<Result<Rc<DynScene>, Rc<MaterialError>>>);
}

impl PyScene {
    pub fn get(&self) -> &PyFuture<Result<Rc<DynScene>, Rc<MaterialError>>> {
        &self.0
    }
}

#[derive(Debug, rpy::FromArgs)]
struct PySceneArgs {
    world: PyObjectRef,
    camera: PyObjectRef,
    width: usize,
    height: usize,
    samples_per_px: u32,
    rays_per_sample: u32,
    ambiant_color: PyVec3,
}

#[rpy::pyimpl]
impl PyScene {
    #[pyslot(new)]
    fn tp_new(_cls: PyClassRef, args: PySceneArgs, vm: &rpy::VirtualMachine) -> PyResult<Self> {
        let pyworld: PyListRef = args.world.try_into_ref(vm)?;
        let pycamera: PyRef<PyCamera> = args.camera.try_into_ref(vm)?;

        let world_futures: Vec<_> = pyworld
            .borrow_elements()
            .iter()
            .map(|py_obj| {
                let shape = <PyRef<PyShape>>::try_from_object(vm, py_obj.clone())?;
                Ok(shape.shared_hit().shared())
            })
            .collect::<PyResult<_>>()?;

        let camera = (*pycamera).clone()
            .builder()
            .dimensions(args.width as f32, args.height as f32)
            .finish();

        let width = args.width;
        let height = args.height;
        let samples_per_px = args.samples_per_px;
        let rays_per_sample = args.rays_per_sample;
        let ambiant_color = args.ambiant_color.into_vec();

        let scene_future = future::try_join_all(world_futures)
            .map_ok(move |world| {
                let scene = Scene {
                    camera,
                    width,
                    height,
                    world: HitList::new(world),
                    samples_per_px,
                    rays_per_sample,
                    ambiant_color
                };
                Rc::new(scene)
            });

        Ok(Self(PyFuture::new(scene_future)))
    }
}
