fn main() {
    let arr = [1,2,3,4,5];

    let slice = &arr[1..3];

    assert_eq!(slice, &[2,3]);
}
