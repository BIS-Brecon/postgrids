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
pub fn osgb_from_eastings_northings(eastings: i32, northings: i32, precision: i32) -> OSGB {
    match gridish::OSGB::new(
        eastings as u32,
        northings as u32,
        parse_precision(precision),
    ) {
        Ok(grid) => OSGB(grid),
        Err(e) => error!("Invalid grid reference: {}", e),
    }
}

#[pg_extern]
pub fn osgb_from_string(string: &str) -> OSGB {
    match gridish::OSGB::from_str(string) {
        Ok(grid) => OSGB(grid),
        Err(e) => error!("Invalid grid reference: {}", e),
    }
}

#[pg_extern]
pub fn osgb_precision(grid: OSGB) -> i32 {
    grid.0.precision().metres() as i32
}

#[pg_extern]
pub fn osgb_recalculate(grid: OSGB, precision: i32) -> OSGB {
    OSGB(grid.0.recalculate(parse_precision(precision)))
}

fn parse_precision(precision: i32) -> Precision {
    match precision {
        1 => Precision::_1M,
        10 => Precision::_10M,
        100 => Precision::_100M,
        1000 => Precision::_1Km,
        10000 => Precision::_10Km,
        100000 => Precision::_100Km,
        _ => error!("{} is not a supported precision.", precision),
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use crate::osgb::*;

    #[pg_test]
    fn test_osgb_from_eastings_northings() {
        assert_eq!(
            "SO892437",
            osgb_from_eastings_northings(389_200, 243_700, 100)
                .0
                .to_string()
        );
    }

    #[pg_test]
    fn test_osgb_from_string() {
        assert_eq!("SO892437", osgb_from_string("SO892437").0.to_string());
    }

    #[pg_test]
    fn test_osgb_precision() {
        assert_eq!(100, osgb_precision(osgb_from_string("SO892437")));
    }

    #[pg_test]
    fn test_osgb_recalculate() {
        assert_eq!(
            osgb_from_string("SO8943"),
            osgb_recalculate(osgb_from_string("SO892437"), 1000)
        );
        assert_eq!(
            osgb_from_string("SO892437"),
            osgb_recalculate(osgb_from_string("SO892437"), 1)
        );
    }
}
