use gridish::Precision;
use pgrx::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PostgresType, PartialEq)]
#[inoutfuncs]
pub struct OSI(gridish::OSI);

impl InOutFuncs for OSI {
    fn input(input: &core::ffi::CStr) -> Self
    where
        Self: Sized,
    {
        let s = match input.to_str() {
            Ok(s) => s,
            Err(e) => error!("{}", e),
        };

        let grid: gridish::OSI = match s.parse() {
            Ok(grid) => grid,
            Err(e) => error!("{}", e),
        };

        OSI(grid)
    }

    fn output(&self, buffer: &mut pgrx::StringInfo) {
        buffer.push_str(&self.0.to_string());
    }
}

#[pg_extern]
fn osi(string: &str) -> OSI {
    match gridish::OSI::from_str(string) {
        Ok(grid) => OSI(grid),
        Err(e) => error!("{}", e),
    }
}

#[pg_extern]
pub fn osi_precision(grid: OSI) -> i32 {
    grid.0.precision().metres() as i32
}

#[pg_extern]
pub fn osi_recalculate(grid: OSI, precision: i32) -> OSI {
    let precision = match precision {
        1 => Precision::_1M,
        10 => Precision::_10M,
        100 => Precision::_100M,
        1000 => Precision::_1Km,
        10000 => Precision::_10Km,
        100000 => Precision::_100Km,
        _ => error!("{} is not a supported precision.", precision),
    };

    OSI(grid.0.recalculate(precision))
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use crate::osi::*;

    #[pg_test]
    fn test_osi_precision() {
        assert_eq!(100, osi_precision(osi("O892437")));
    }

    #[pg_test]
    fn test_osi_recalculate() {
        assert_eq!(osi("O8943"), osi_recalculate(osi("O892437"), 1000));
        assert_eq!(osi("O892437"), osi_recalculate(osi("O892437"), 1));
    }
}
