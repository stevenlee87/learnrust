use std::iter::zip;

fn main(){
    let xs = [1, 2, 3];
    let ys = [4, 5, 6];

    let mut iter = zip(xs, ys);

    assert_eq!(iter.next().unwrap(), (1, 4));
    assert_eq!(iter.next().unwrap(), (2, 5));
    assert_eq!(iter.next().unwrap(), (3, 6));
    assert!(iter.next().is_none());
}