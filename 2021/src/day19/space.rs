use ndarray::Array1;

pub fn distance(point1: &Array1<i32>, point2: &Array1<i32>) -> f64 {
  (0..2)
    .into_iter()
    .map(|d| point1[d] - point2[d])
    .map(|l| l as f64)
    .map(|l| l.powf(2f64))
    .sum::<f64>()
    .sqrt()
}
