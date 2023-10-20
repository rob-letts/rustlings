#[test]
fn main() {
  let mut vec1 = fill_vec();
  assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec() -> Vec<i32> {
    let mut vec = vec![22, 44, 66];
    vec.push(88);
    vec
}
