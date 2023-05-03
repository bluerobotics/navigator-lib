use rand::Rng;

#[cfg(feature = "python")]
use pyo3::{prelude::*, wrap_pyfunction};

use cpy_binder::export_cpy;

#[rustfmt::skip]
export_cpy!(
    navigator {
        enum Material {
            Plastic,
            Rubber,
        }

        struct Size2D {
            width: f64,
            height: f64,
        }

        struct Tire {
            material: Material,
            pressure: f64,
            size: Size2D,
        }

        fn create_random_tire() -> Tire {
            let mut rng = rand::thread_rng();
            Tire {
                material: Material::Plastic,
                pressure: rng.gen_range(30.0..60.0),
                size: Size2D {
                    width: rng.gen_range(5.0..10.0),
                    height: rng.gen_range(10.0..20.0),
                },
            }
        }
    }
);
