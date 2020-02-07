use trt_core::{
    hit::{HitList, RectBuilder},
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
    rays_per_px: usize,
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

        let mut world: Vec<_> = pyworld
            .borrow_elements()
            .iter()
            .map(|py_obj| {
                extract_hit(vm, py_obj.clone()).map(|s| (*s).clone())
            })
            .collect::<PyResult<_>>()?;

        // world.push(Box::new(
        //     RectBuilder
        //         .x(-100..=100)
        //         .z(-100..=100)
        //         .y(500)
        //         .diffuse_color((7, 7, 7)),
        // ));
        world.push(Rc::new(
            RectBuilder.x(113..=443).z(127..=432).y(554).diffuse_color((7, 7, 7))
        ));
        // world.push(Box::new(
        //     RectBuilder.x(0..=555).z(0..=555).y(555).matte((1,1,1)).flip_normals(),
        // ));

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

        Ok(Self(Rc::new(scene)))
    }
}
