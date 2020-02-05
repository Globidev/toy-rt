use rustpython_vm as rpy;
use rpy::{
    pyobject::{PyObjectRef, PyResult, TryFromObject},
    obj::objfloat::try_float,
    VirtualMachine,
};

#[derive(Debug, Clone, Copy)]
pub struct FloatLike(f64);

impl FloatLike {
    pub fn as_f32(self) -> f32 {
        self.0 as _
    }

    pub fn as_f64(self) -> f64 {
        self.0
    }
}

fn extract_f64(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<f64> {
    try_float(obj, vm)?
        .ok_or_else(|| {
            let value_as_str = vm.to_pystr(obj)
                .unwrap_or_else(|_| String::from("Unknown value"));

            let error_msg = format!(
                "Expected a numeric value, got a value of type '{}': '{}'",
                obj.typ.name,
                value_as_str
            );

            vm.new_type_error(error_msg)
        })
}

impl TryFromObject for FloatLike {
    fn try_from_object(vm: &VirtualMachine, obj: PyObjectRef) -> PyResult<Self> {
        Ok(FloatLike(extract_f64(&obj, vm)?))
    }
}
