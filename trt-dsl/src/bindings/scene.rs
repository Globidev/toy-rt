use trt_core::{
    hit::{HitList, RectBuilder},
    prelude::*,
    scene::Scene,
};

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, PyValue, TryIntoRef},
};

use super::{camera::PyCamera, sphere::PySphere};
use rpy::{obj::objlist::PyListRef, pyobject::{PyRef, PyObjectRef}};
use std::{cell::RefCell, fmt};

pub type DynScene = Scene<HitList<Box<dyn Hit>>>;

#[rpy::pyclass(name = "Scene")]
pub struct PyScene(RefCell<Option<DynScene>>);

impl fmt::Debug for PyScene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Scene>")
    }
}

impl PyScene {
    pub fn take(&self) -> DynScene {
        std::mem::take(&mut *self.0.borrow_mut()).expect("Attempted to move out of an empty scene")
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
    rays_per_px: usize,
}

#[rpy::pyimpl]
impl PyScene {
    #[pyslot(new)]
    fn tp_new(_cls: PyClassRef, args: PySceneArgs, vm: &rpy::VirtualMachine) -> PyResult<Self> {
        let pyworld: PyListRef = args.world.try_into_ref(vm)?;
        let pycamera: PyRef<PyCamera> = args.camera.try_into_ref(vm)?;

        let mut world: Vec<_> = pyworld
            .borrow_elements()
            .iter()
            .map(|py_obj| {
                let as_sphere: PyRef<PySphere> = py_obj.clone().try_into_ref(vm)?;
                Ok(Box::new((*as_sphere).clone().into_hit()) as Box<dyn Hit>)
            })
            .collect::<PyResult<_>>()?;

        world.push(Box::new(
            RectBuilder
                .x(-100..=100)
                .z(-100..=100)
                .y(500)
                .diffuse_color((7, 7, 7)),
        ));

        let camera = (*pycamera).clone()
            .builder()
            .dimensions(args.width as f32, args.height as f32)
            .finish();

        let scene = Scene {
            camera,
            width: args.width,
            height: args.height,
            world: HitList::new(world),
            ray_per_px: args.rays_per_px,
        };

        Ok(Self(RefCell::new(Some(scene))))
    }
}
