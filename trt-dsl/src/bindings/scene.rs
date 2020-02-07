use trt_core::{
    camera::CameraBuilder,
    hit::{HitList, RectBuilder},
    prelude::*,
    scene::Scene,
};

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, PyValue},
};

use super::sphere::PySphere;
use rpy::{obj::objlist::PyList, pyobject::PyObjectRef};
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
    width: usize,
    height: usize,
    rays_per_px: usize,
}

#[rpy::pyimpl]
impl PyScene {
    #[pyslot(new)]
    fn tp_new(_cls: PyClassRef, args: PySceneArgs, _vm: &rpy::VirtualMachine) -> PyResult<Self> {
        let as_py_list = args.world.downcast::<PyList>()?;

        let mut world: Vec<_> = as_py_list
            .elements
            .borrow()
            .iter()
            .map(|py_obj| {
                let as_sphere = py_obj.clone().downcast::<PySphere>()?;
                Ok(Box::new((*as_sphere).clone().into_hit()) as Box<dyn Hit>)
            })
            .collect::<PyResult<_>>()?;

        world.push(Box::new(
            RectBuilder
                .x(-50..=50)
                .z(-50..=50)
                .y(50)
                .diffuse_color((7, 7, 7)),
        ));

        let camera = CameraBuilder::default()
            .look_from((278, 278, -800))
            .look_at((278, 278, 0))
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
