use crate::kernels::vadd;

mod kernels;

fn main() {
    let a: Vec<f32> = (0..10_000).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..10_000).map(|i| (2 * i) as f32).collect();
    let c = vadd(&a, &b);
    for i in 0..a.len() {
        assert_eq!(c[i], a[i] + b[i]);
    }
}
