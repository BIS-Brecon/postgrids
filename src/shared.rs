use gridish::Precision;
use pgrx::error;

pub fn parse_precision(precision: i32) -> Precision {
    match precision {
        1 => Precision::_1M,
        10 => Precision::_10M,
        100 => Precision::_100M,
        1000 => Precision::_1Km,
        2000 => Precision::_2Km,
        10000 => Precision::_10Km,
        100000 => Precision::_100Km,
        _ => error!("{} is not a supported precision.", precision),
    }
}
