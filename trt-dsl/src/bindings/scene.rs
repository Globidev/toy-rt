use trt_core::{
    hit::HitList,
    prelude::*,
    scene::Scene,
};

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, PyValue, TryIntoRef, TryFromObject},
};

use super::{camera::PyCamera, vec3::PyVec3, shape::PyShape};
use rpy::{obj::objlist::PyListRef, pyobject::{PyRef, PyObjectRef}};
use std::{fmt, rc::Rc};

use crate::future::PyFuture;
use futures::prelude::*;

pub type DynScene = Scene<HitList<Rc<dyn Hit>>>;

#[rpy::pyclass(name = "Scene")]
pub struct PyScene(PyFuture<Rc<DynScene>>);

impl fmt::Debug for PyScene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Scene>")
    }
}

impl PyScene {
    pub fn get(&self) -> &PyFuture<Rc<DynScene>> {
        &self.0
    }
}

impl PyValue for PyScene {
    fn class(vm: &rpy::VirtualMachine) -> PyClassRef {
        vm.class(super::TRT_MODULE_NAME, "Scene")
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
                Ok(shape.shared_hit().get().shared())
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

        let scene_future = future::join_all(world_futures)
            .map(move |world| {
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
