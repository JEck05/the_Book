fn main() {
    
}
// this demonstrates how the iterator works
// and how it is consumed at the end of use
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
// This uses(takes ownership) up the iterator because .sum() calls next
// until None and returns the times called
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
#[test]
fn iterator_mapping(){
    let v1 = vec![1,2,3];
    
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    
    assert_eq!(v2, vec![2,3,4]);
    
}
