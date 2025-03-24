use std::str::FromStr;

pub trait Bounded {
    type ValueType: Ord
        + PartialOrd
        + Copy
        + FromStr
        + std::fmt::Display
        + std::fmt::Debug
        + std::ops::Add<Output = Self::ValueType>
        + std::ops::Sub<Output = Self::ValueType>;

    fn min_value(&self) -> Self::ValueType;
    fn max_value(&self) -> Self::ValueType;
    fn increment(&self) -> Self::ValueType;
}
