use std::ops::AddAssign;

pub fn mean<T>(vec: Vec<T>) -> f64 
where 
    f64:AddAssign<T>,
    T:Copy,
{
    let mut sum = 0_f64; // -> default value for summision
    for i in &vec {
        sum += *i;
    }
    sum / vec.len() as f64
}
