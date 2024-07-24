use pgrx::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::shared::parse_precision;

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
pub fn osi_from_eastings_northings(eastings: i32, northings: i32, precision: i32) -> OSI {
    match gridish::OSI::new(
        eastings as u32,
        northings as u32,
        parse_precision(precision),
    ) {
        Ok(grid) => OSI(grid),
        Err(e) => error!("Invalid grid reference: {}", e),
    }
}

#[pg_extern]
pub fn osi_from_string(string: &str) -> OSI {
    match gridish::OSI::from_str(string) {
        Ok(grid) => OSI(grid),
        Err(e) => error!("Invalid grid reference: {}", e),
    }
}

#[pg_extern]
pub fn osi_precision(grid: OSI) -> i32 {
    grid.0.precision().metres() as i32
}

#[pg_extern]
pub fn osi_recalculate(grid: OSI, precision: i32) -> OSI {
    OSI(grid.0.recalculate(parse_precision(precision)))
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use crate::osi::*;

    #[pg_test]
    fn test_osi_from_eastings_northings() {
        assert_eq!(
            "O892437",
            osi_from_eastings_northings(389_200, 243_700, 100)
                .0
                .to_string()
        );
    }

    #[pg_test]
    fn test_osi_from_string() {
        assert_eq!("O892437", osi_from_string("O892437").0.to_string());
    }

    #[pg_test]
    fn test_osi_precision() {
        assert_eq!(100, osi_precision(osi_from_string("O892437")));
    }

    #[pg_test]
    fn test_osi_recalculate() {
        assert_eq!(
            osi_from_string("O8943"),
            osi_recalculate(osi_from_string("O892437"), 1000)
        );
        assert_eq!(
            osi_from_string("O892437"),
            osi_recalculate(osi_from_string("O892437"), 1)
        );
    }
}
