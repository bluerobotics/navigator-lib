#[macro_export]
macro_rules! export_cpy {
    ($(enum $enum_name:ident { $($enum_variant:ident,)* })+
     $(struct $struct_name:ident { $($struct_field:ident : $struct_field_type:ty,)* })+
     $(fn $func_name:ident() -> $func_ret:ty $func_body:block)+
     module_name: $module_name:ident) => {
        $(
            export_cpy!(enum $enum_name { $($enum_variant,)* });
        )+

        $(
            export_cpy!(struct $struct_name { $($struct_field : $struct_field_type,)* });
        )+

        $(
            export_cpy!(fn $func_name() -> $func_ret $func_body);
        )+

        #[cfg(feature = "python")]
        #[pymodule]
        fn $module_name(_py: Python, m: &PyModule) -> PyResult<()> {
            $(
                m.add_class::<$struct_name>()?;
            )+
            $(
                m.add_class::<$enum_name>()?;
            )+
            $(
                m.add_wrapped(wrap_pyfunction!($func_name))?;
            )+
            Ok(())
        }
    };
    (enum $name:ident { $($variant:ident,)* }) => {
        #[derive(Clone, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
        pub enum $name {
            $(
                $variant,
            )*
        }
    };
    (struct $name:ident { $($field:ident : $ftype:ty,)* }) => {
        #[derive(Clone, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, set_all))]
        pub struct $name {
            $(
                pub $field: $ftype,
            )*
        }
    };
    ($(fn $name:ident() -> $ret:ty $body:block)+) => {
        $(
            #[no_mangle]
            #[cfg_attr(feature = "python", pyfunction)]
            pub extern "C" fn $name() -> $ret {
                $body
            }
        )+
    };
}
