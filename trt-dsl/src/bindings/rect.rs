use trt_core::{
    hit::{RectBuilder},
    prelude::*,
};

use rustpython_vm::{
    self as rpy,
    obj::objtype::PyClassRef,
    pyobject::{PyResult, PyValue, TryFromObject},
};

use super::{float::FloatLike, SharedHit, material::PyMaterial};
use std::{ops::RangeInclusive};
use rpy::{obj::objtuple::PyTupleRef, pyobject::PyObjectRef};

#[rpy::pyclass(name = "Rect")]
#[derive(Debug)]
pub struct PyRect(SharedHit);

impl PyRect {
    pub fn shared_hit(&self) -> SharedHit {
        self.0.clone()
    }
}

impl PyValue for PyRect {
    fn class(vm: &rpy::VirtualMachine) -> PyClassRef {
        vm.class(super::TRT_MODULE_NAME, "Rect")
    }
}

#[derive(Debug, rpy::FromArgs)]
struct PyRectArgs {
    x: PyObjectRef,
    y: PyObjectRef,
    z: PyObjectRef,
    material: PyMaterial,
}

#[derive(Debug)]
enum FloatOrRange {
    Float(f32),
    Range(RangeInclusive<f32>),
}

impl FloatOrRange {
    fn range(&self) -> Option<RangeInclusive<f32>> {
        match self {
            FloatOrRange::Float(_) => None,
            FloatOrRange::Range(r) => Some(*r.start()..=*r.end()),
        }
    }

    fn float(&self) -> Option<f32> {
        match self {
            FloatOrRange::Float(f) => Some(*f),
            FloatOrRange::Range(_) => None,
        }
    }
}

fn float_or_range(vm: &rpy::VirtualMachine, obj_ref: PyObjectRef) -> PyResult<FloatOrRange> {
    match FloatLike::try_from_object(vm, obj_ref.clone()) {
        Ok(f) => Ok(FloatOrRange::Float(f.as_f32())),
        Err(_) => {
            let tuple = PyTupleRef::try_from_object(vm, obj_ref)?;
            let as_slice = tuple.as_slice();
            let start = FloatLike::try_from_object(vm, as_slice[0].clone())?.as_f32();
            let end = FloatLike::try_from_object(vm, as_slice[1].clone())?.as_f32();
            Ok(FloatOrRange::Range(start..=end))
        },
    }
}

#[rpy::pyimpl]
impl PyRect {
    #[pyslot(new)]
    fn tp_new(_cls: PyClassRef, args: PyRectArgs, vm: &rpy::VirtualMachine) -> PyResult<Self> {
        let x = float_or_range(vm, args.x)?;
        let y = float_or_range(vm, args.y)?;
        let z = float_or_range(vm, args.z)?;
        let material = args.material.shared_material();

        let rect = match x {
            FloatOrRange::Float(x) => {
                let y_range = y.range().ok_or_else(|| vm.new_value_error(format!("")))?;
                let z_range = z.range().ok_or_else(|| vm.new_value_error(format!("")))?;
                SharedHit::new(RectBuilder.y(y_range).z(z_range).x(x).material(material)) as _
            },
            FloatOrRange::Range(x_range) => {
                match y {
                    FloatOrRange::Float(y) => {
                        let z_range = z.range().ok_or_else(|| vm.new_value_error(format!("")))?;
                        SharedHit::new(RectBuilder.x(x_range).z(z_range).y(y).material(material)) as _
                    },
                    FloatOrRange::Range(y_range) => {
                        let z = z.float().ok_or_else(|| vm.new_value_error(format!("")))?;
                        SharedHit::new(RectBuilder.x(x_range).y(y_range).z(z).material(material)) as _
                    },
                }
            },
        };

        Ok(Self(rect))
    }

    #[pymethod]
    fn flip_normals(&self) -> PyRect {
        Self(SharedHit::new((*self.shared_hit()).clone().flip_normals()))
    }
}
