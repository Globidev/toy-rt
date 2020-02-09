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

use super::{camera::PyCamera, sphere::PySphere, rect::PyRect, SharedHit};
use rpy::{obj::objlist::PyListRef, pyobject::{PyRef, PyObjectRef}};
use std::{fmt, rc::Rc};

pub type DynScene = Scene<HitList<Rc<dyn Hit>>>;

#[rpy::pyclass(name = "Scene")]
pub struct PyScene(Rc<DynScene>);

impl fmt::Debug for PyScene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Scene>")
    }
}

impl PyScene {
    pub fn shared(&self) -> Rc<DynScene> {
        self.0.clone()
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
}

fn extract_hit(vm: &rpy::VirtualMachine, obj: PyObjectRef) -> PyResult<SharedHit> {
    match <PyRef<PySphere>>::try_from_object(vm, obj.clone()) {
        Ok(r) => Ok(r.shared_hit()),
        Err(_) => {
            let r = <PyRef<PyRect>>::try_from_object(vm, obj)?;
            Ok(r.shared_hit())
        },
    }
}

#[rpy::pyimpl]
impl PyScene {
    #[pyslot(new)]
    fn tp_new(_cls: PyClassRef, args: PySceneArgs, vm: &rpy::VirtualMachine) -> PyResult<Self> {
        let pyworld: PyListRef = args.world.try_into_ref(vm)?;
        let pycamera: PyRef<PyCamera> = args.camera.try_into_ref(vm)?;

        let world: Vec<_> = pyworld
            .borrow_elements()
            .iter()
            .map(|py_obj| {
                extract_hit(vm, py_obj.clone()).map(|s| (*s).clone())
            })
            .collect::<PyResult<_>>()?;

        let camera = (*pycamera).clone()
            .builder()
            .dimensions(args.width as f32, args.height as f32)
            .finish();

        let scene = Scene {
            camera,
            width: args.width,
            height: args.height,
            world: HitList::new(world),
            samples_per_px: args.samples_per_px,
            rays_per_sample: args.rays_per_sample,
        };

        Ok(Self(Rc::new(scene)))
    }
}
