use gridish::Precision;
use pgrx::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PostgresType, PartialEq)]
#[inoutfuncs]
pub struct OSGB(gridish::OSGB);

impl InOutFuncs for OSGB {
    fn input(input: &core::ffi::CStr) -> Self
    where
        Self: Sized,
    {
        let s = match input.to_str() {
            Ok(s) => s,
            Err(e) => error!("{}", e),
        };

        let grid: gridish::OSGB = match s.parse() {
            Ok(grid) => grid,
            Err(e) => error!("{}", e),
        };

        OSGB(grid)
    }

    fn output(&self, buffer: &mut pgrx::StringInfo) {
        buffer.push_str(&self.0.to_string());
    }
}

#[pg_extern]
fn osgb(string: &str) -> OSGB {
    match gridish::OSGB::from_str(string) {
        Ok(grid) => OSGB(grid),
        Err(e) => error!("{}", e),
    }
}

#[pg_extern]
pub fn osgb_precision(grid: OSGB) -> i32 {
    grid.0.precision().metres() as i32
}

#[pg_extern]
pub fn osgb_recalculate(grid: OSGB, precision: i32) -> OSGB {
    let precision = match precision {
        1 => Precision::_1M,
        10 => Precision::_10M,
        100 => Precision::_100M,
        1000 => Precision::_1Km,
        10000 => Precision::_10Km,
        100000 => Precision::_100Km,
        _ => error!("{} is not a supported precision.", precision),
    };

    OSGB(grid.0.recalculate(precision))
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use crate::osgb::*;

    #[pg_test]
    fn test_osgb_precision() {
        assert_eq!(100, osgb_precision(osgb("SO892437")));
    }

    #[pg_test]
    fn test_osgb_recalculate() {
        assert_eq!(osgb("SO8943"), osgb_recalculate(osgb("SO892437"), 1000));
        assert_eq!(osgb("SO892437"), osgb_recalculate(osgb("SO892437"), 1));
    }
}
