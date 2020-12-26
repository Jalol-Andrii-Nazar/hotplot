use iced_core::{Point, Rectangle, Size};
use iced_graphics::Color;
use std::{cmp::Ordering, hash::Hash};

#[cfg(feature = "chrono")]
use chrono::{Date, DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, offset::TimeZone};

pub struct Settings {
    pub background_color: Color,
    pub padded_background_color: Color,
    pub margined_background_color: Option<Color>,
    pub title: String,
    pub title_color: Color,
    pub title_size: f32,
    pub padding: QuadDistance,
    pub margin: QuadDistance,
    pub min_x_label_distance: DistanceValue,
    pub min_y_label_distance: DistanceValue,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background_color: Color::from_rgb8(211, 211, 211),
            padded_background_color: Color::WHITE,
            margined_background_color: Some(Color::from_rgb8(241, 241, 241)),
            title: "A plot".to_string(),
            title_color: Color::BLACK,
            title_size: 32.0,
            padding: QuadDistance::from1(DistanceValue::Fixed(60.0)),
            margin: QuadDistance::from1(DistanceValue::Fixed(20.0)),
            min_x_label_distance: DistanceValue::Fixed(100.0),
            min_y_label_distance: DistanceValue::Fixed(50.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlotSettings {
    pub color: Color,
    pub line_selection_distance: f32,
    pub point_selection_distance: f32,
    pub line_size1: f32,  //Line is not selected
    pub line_size2: f32,  //Line is selected
    pub point_size1: f32, //Point is not selected
    pub point_size2: f32, //Point is selected inderectly (through a selected line)
    pub point_size3: f32, //Point is selected directly
}

impl Default for PlotSettings {
    fn default() -> Self {
        Self {
            color: Color::from_rgb8(200, 0, 0),
            line_selection_distance: 4.0,
            point_selection_distance: 10.0,
            line_size1: 2.0,
            line_size2: 3.0,
            point_size1: 5.0,
            point_size2: 7.0,
            point_size3: 10.0,
        }
    }
}

impl PartialEq for PlotSettings {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && self.line_size1 == other.line_size1
            && self.line_size2 == other.line_size2
            && self.point_size1 == other.point_size1
            && self.point_size2 == other.point_size2
            && self.point_size3 == other.point_size3
    }
}

impl Eq for PlotSettings {}

impl Hash for PlotSettings {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.color.r.to_bits());
        state.write_u32(self.color.g.to_bits());
        state.write_u32(self.color.b.to_bits());
        state.write_u32(self.color.a.to_bits());
        state.write_u32(self.line_size1.to_bits());
        state.write_u32(self.line_size2.to_bits());
        state.write_u32(self.point_size1.to_bits());
        state.write_u32(self.point_size2.to_bits());
        state.write_u32(self.point_size3.to_bits());
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DistanceValue {
    Fixed(f32),
    Relative(fn(Size) -> f32),
}

impl DistanceValue {
    pub fn get(&self, size: Size) -> f32 {
        match self {
            DistanceValue::Fixed(value) => *value,
            DistanceValue::Relative(f) => f(size),
        }
    }
}

pub struct QuadDistance {
    pub top: DistanceValue,
    pub right: DistanceValue,
    pub bottom: DistanceValue,
    pub left: DistanceValue,
}

impl QuadDistance {
    pub fn from4(
        first: DistanceValue,
        second: DistanceValue,
        third: DistanceValue,
        fourth: DistanceValue,
    ) -> Self {
        Self {
            top: first,
            right: second,
            bottom: third,
            left: fourth,
        }
    }

    pub fn from3(first: DistanceValue, second: DistanceValue, third: DistanceValue) -> Self {
        Self {
            top: first,
            right: second,
            bottom: third,
            left: second,
        }
    }

    pub fn from2(first: DistanceValue, second: DistanceValue) -> Self {
        Self {
            top: first,
            right: second,
            bottom: first,
            left: second,
        }
    }

    pub fn from1(first: DistanceValue) -> Self {
        Self {
            top: first,
            right: first,
            bottom: first,
            left: first,
        }
    }

    pub fn get(&self, size: Size) -> (f32, f32, f32, f32) {
        let top = self.top.get(size);
        let right = self.right.get(size);
        let bottom = self.bottom.get(size);
        let left = self.left.get(size);
        (top, right, bottom, left)
    }

    pub fn transform(&self, area: Rectangle) -> Rectangle {
        let position = area.position();
        let x = position.x;
        let y = position.y;
        let size = area.size();
        let width = size.width;
        let height = size.height;
        let (top, right, bottom, left) = self.get(size);
        let new_position = Point::new(x + left, y + top);
        let new_size = Size::new(width - left - right, height - top - bottom);
        Rectangle::new(new_position, new_size)
    }
}

#[derive(Debug, Clone)]
pub struct Message {}

pub trait AxisValue: Clone {
    fn compare_value(&self, other: &Self) -> Ordering;
    //self <= other
    fn distance_to(&self, other: &Self) -> f32;
    fn add(&self, value: f32) -> Option<Self>;
    //self <= other, 0 < min_distance <= optimal_distance
    fn get_values_in_between(
        &self,
        other: &Self,
        min_distance: f32,
        optimal_distance: f32,
    ) -> Vec<Self>;
}

pub trait AxisData<V: AxisValue>: Clone {
    fn value(&self) -> &V;

    fn display_value(value: &V) -> String;

    fn description(&self) -> String {
        Self::display_value(self.value())
    }
}

macro_rules! integer_axis_value_impl {
    ($($x:ident),*) => {
        $(
            impl AxisValue for $x {

                fn compare_value(&self, other: &Self) -> Ordering {
                    self.cmp(other)
                }

                fn distance_to(&self, other: &Self) -> f32 {
                    assert!(*self <= *other);
                    (*other - *self) as f32
                }

                fn add(&self, value: f32) -> Option<Self> {
                    self.checked_add(value as $x)
                }

                fn get_values_in_between(&self, other: &Self, min_distance: f32, optimal_distance: f32) -> Vec<Self> {
                    assert!(*self <= *other);
                    let mut result: Vec<Self> = Vec::new();
                    for i in 1.. {
                        let perfect_value: f32 = *self as f32 + optimal_distance * i as f32;
                        let value: $x = perfect_value as $x;
                        if value as f32 + min_distance < *other as f32 {
                            result.push(value);
                        } else {
                            break;
                        }
                    }
                    result
                }
            }
        )*
    };
}

integer_axis_value_impl!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

macro_rules! float_axis_value_and_data_impl {
    ($($x:ident),*) => {
        $(
            impl AxisValue for $x {

                fn compare_value(&self, other: &Self) -> Ordering {
                    self.total_cmp(other)
                }

                fn distance_to(&self, other: &Self) -> f32 {
                    assert!(*self <= *other);
                    (*other - *self) as f32
                }

                fn add(&self, value: f32) -> Option<Self> {
                    Some(*self + value as $x)
                }

                fn get_values_in_between(&self, other: &Self, min_distance: f32, optimal_distance: f32) -> Vec<Self> {
                    assert!(*self <= *other);
                    let mut result: Vec<Self> = Vec::new();
                    for i in 1.. {
                        let value = *self + (i as f32 * optimal_distance) as $x;
                        if value as f32 + min_distance < *other as f32 {
                            result.push(value);
                        } else {
                            break;
                        }
                    }
                    result
                }
            }

            impl AxisData<$x> for $x {
                fn value(&self) -> &$x {
                    &self
                }

                fn display_value(value: &$x) -> String {
                    format!("{:.2}", value)
                }
            }
        )*
    };
}

float_axis_value_and_data_impl!(f32, f64);

impl AxisValue for char {
    fn compare_value(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }

    fn distance_to(&self, other: &Self) -> f32 {
        let self_u32 = *self as u32;
        let other_u32 = *other as u32;
        assert!(self_u32 <= other_u32);
        (other_u32 - self_u32) as f32
    }

    fn add(&self, value: f32) -> Option<Self> {
        std::char::from_u32(*self as u32 + value as u32)
    }

    fn get_values_in_between(
        &self,
        other: &Self,
        min_distance: f32,
        optimal_distance: f32,
    ) -> Vec<Self> {
        Vec::new()
    }
}

impl AxisValue for bool {
    fn compare_value(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }

    fn distance_to(&self, other: &Self) -> f32 {
        assert!(*self <= *other);
        if *self == *other {
            0.0
        } else {
            1.0
        }
    }

    fn add(&self, value: f32) -> Option<Self> {
        if *self == false && value >= 1.0 {
            Some(true)
        } else {
            None
        }
    }

    fn get_values_in_between(
        &self,
        _other: &Self,
        _min_distance: f32,
        _optimal_distance: f32,
    ) -> Vec<Self> {
        Vec::new()
    }
}

#[cfg(feature = "chrono")]
macro_rules! time_axis_value_impl {
    ($($x:ident),*) => {
        $(
            impl AxisValue for $x {
                fn compare_value(&self, other: &Self) -> Ordering {
                    self.cmp(other)
                }
            
                fn distance_to(&self, other: &Self) -> f32 {
                    (*other - *self).num_milliseconds() as f32
                }
            
                fn add(&self, value: f32) -> Option<Self> {
                    Some(*self + chrono::Duration::milliseconds(value as i64))
                }
            
                fn get_values_in_between(
                    &self,
                    other: &Self,
                    min_distance: f32,
                    optimal_distance: f32,
                ) -> Vec<Self> {
                    let mut result: Vec<Self> = Vec::new();
                    for i in 1.. {
                        let value: $x =
                            *self + Duration::milliseconds((optimal_distance * i as f32) as i64);
                        if value + Duration::milliseconds(min_distance as i64) < *other {
                            result.push(value);
                        } else {
                            break;
                        }
                    }
                    result
                }
            }
        )*
    };
}

#[cfg(feature = "chrono")]
time_axis_value_impl!(NaiveTime, NaiveDateTime);

#[cfg(feature = "chrono")]
macro_rules! tz_time_axis_value_impl {
    ($($x:ident),*) => {
        $(
            impl <Tz: TimeZone> AxisValue for $x<Tz>
            where <Tz as TimeZone>::Offset: Copy
            {
                fn compare_value(&self, other: &Self) -> Ordering {
                    self.cmp(other)
                }
            
                fn distance_to(&self, other: &Self) -> f32 {
                    (*other - *self).num_milliseconds() as f32
                }
            
                fn add(&self, value: f32) -> Option<Self> {
                    Some(*self + chrono::Duration::milliseconds(value as i64))
                }
            
                fn get_values_in_between(
                    &self,
                    other: &Self,
                    min_distance: f32,
                    optimal_distance: f32,
                ) -> Vec<Self> {
                    let mut result: Vec<Self> = Vec::new();
                    for i in 1.. {
                        let value: $x<Tz> =
                            *self + Duration::milliseconds((optimal_distance * i as f32) as i64);
                        if value + Duration::milliseconds(min_distance as i64) < *other {
                            result.push(value);
                        } else {
                            break;
                        }
                    }
                    result
                }
            }
        )*
    };
}

#[cfg(feature = "chrono")]
tz_time_axis_value_impl!(DateTime);

#[cfg(feature = "chrono")]
macro_rules! date_axis_value_impl {
    ($($x:ident),*) => {
        $(
            impl AxisValue for $x {
                fn compare_value(&self, other: &Self) -> Ordering {
                    self.cmp(other)
                }
            
                fn distance_to(&self, other: &Self) -> f32 {
                    (*other - *self).num_days() as f32
                }
            
                fn add(&self, value: f32) -> Option<Self> {
                    Some(*self + chrono::Duration::days(value as i64))
                }
            
                fn get_values_in_between(
                    &self,
                    other: &Self,
                    min_distance: f32,
                    optimal_distance: f32,
                ) -> Vec<Self> {
                    let mut result: Vec<Self> = Vec::new();
                    for i in 1.. {
                        let value: chrono::NaiveDate =
                            *self + Duration::days((optimal_distance * i as f32) as i64);
                        if value + Duration::days(min_distance as i64) < *other {
                            result.push(value);
                        } else {
                            break;
                        }
                    }
                    result
                }
            }
        )*
    };
}

#[cfg(feature = "chrono")]
date_axis_value_impl!(NaiveDate);

#[cfg(feature = "chrono")]
macro_rules! tz_date_axis_value_impl {
    ($($x:ident),*) => {
        $(
            impl <Tz: TimeZone> AxisValue for $x<Tz>
            where <Tz as TimeZone>::Offset: Copy
            {
                fn compare_value(&self, other: &Self) -> Ordering {
                    self.cmp(other)
                }
            
                fn distance_to(&self, other: &Self) -> f32 {
                    (*other - *self).num_days() as f32
                }
            
                fn add(&self, value: f32) -> Option<Self> {
                    Some(*self + chrono::Duration::days(value as i64))
                }
            
                fn get_values_in_between(
                    &self,
                    other: &Self,
                    min_distance: f32,
                    optimal_distance: f32,
                ) -> Vec<Self> {
                    let mut result: Vec<Self> = Vec::new();
                    for i in 1.. {
                        let value: $x<Tz> =
                            *self + Duration::days((optimal_distance * i as f32) as i64);
                        if value + Duration::days(min_distance as i64) < *other {
                            result.push(value);
                        } else {
                            break;
                        }
                    }
                    result
                }
            }
        )*
    };
}

#[cfg(feature = "chrono")]
tz_date_axis_value_impl!(Date);

macro_rules! default_axis_data_impl {
    ($($x:ident),*) => {
        $(
            impl AxisData<$x> for $x {
                fn value(&self) -> &$x {
                    &self
                }

                fn display_value(value: &$x) -> String {
                    format!("{:?}", value)
                }
            }
        )*
    };
}

default_axis_data_impl!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, char, bool);

#[cfg(feature = "chrono")]
default_axis_data_impl!(NaiveTime, NaiveDateTime, NaiveDate);
