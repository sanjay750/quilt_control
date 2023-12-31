

extern crate num;

use num::complex::Complex;
use std::vec::Vec;




pub enum ThreadColor {
    Red,
    Blue,
    Green,
    Black,
}

//--------------------------------Line
pub struct Line {
    pub vect:Complex<f64>,
    pub color:ThreadColor,
}

trait Chopabel {
    fn chop(&self, chop_size:f64) -> Vec<Line>;
}

impl Chopabel for Line {
    fn chop(&self, chop_size:f64) -> Vec<Line> {
        let vlist :Vec<Line> = vec![Line::new(0.0,0.0, ThreadColor::Red)];
        vlist
    }
}

impl Line {

    pub fn new(lx: f64, ly: f64, color_:ThreadColor) -> Line {
        Line{vect: Complex::new(lx, ly), color: color_} 
    }

    pub fn get_len(&self) -> f64 {
        self.vect.to_polar().0
    }

    pub fn get_angle(&self) -> f64 {
        self.vect.to_polar().1
    }
}

//------------------------------- Line List
pub struct LineList {
    l_list:Vec<Complex<f64>>,
    color:ThreadColor,
}

//-------------------------------Arc

pub struct Arc {

    pub cord:Complex<f64>,
    pub ctrl:Complex<f64>,
    pub color:ThreadColor,
}

impl Arc {

    pub fn new(ex:f64, ey:f64, cx:f64, cy:f64, color_:ThreadColor) -> Arc {
        Arc{cord: Complex::new(ex, ey), ctrl: Complex::new(cx, cy), color:color_}
    }
}

