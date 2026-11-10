
pub trait EdgeWeight{
    fn weight(&self) ->f64;
}

impl EdgeWeight for i32 { fn weight(&self) -> f64 { *self as f64 } }
impl EdgeWeight for u32 { fn weight(&self) -> f64 { *self as f64 } }
impl EdgeWeight for i64 { fn weight(&self) -> f64 { *self as f64 } }
impl EdgeWeight for u64 { fn weight(&self) -> f64 { *self as f64 } }
impl EdgeWeight for f32 { fn weight(&self) -> f64 { *self as f64 } }
impl EdgeWeight for f64 { fn weight(&self) -> f64 { *self } }
impl EdgeWeight for usize { fn weight(&self) -> f64 { *self as f64 } }
impl EdgeWeight for i16 { fn weight(&self) -> f64 { *self as f64 } }
impl EdgeWeight for u16 { fn weight(&self) -> f64 { *self as f64 } }
impl EdgeWeight for () {fn weight(&self) -> f64 { 1.0 } }
