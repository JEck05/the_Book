#[derive(Debug, PartialEq)]
#[derive(Clone)]
struct Shoe{
    size: u32,
    style: String,
}
// into_iter consumes the shoes Vector and returns just the
// items that match the closure, in a new vector
// this avoids copying but consumes the given vector
// to avoid consumption you could use references aswell(see below)
fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
// this creates a Vector (without copying) from the given reference vector
// this means that when the new vector goes out of scope the new one can be called
// as it is not "consumed"
fn shoe_in_size_references(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<&Shoe> {
    shoes.iter().filter(|s| s.size == shoe_size).collect()
}
fn main() {
    
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn filter_by_size_reference(){
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe{
                size: 11,
                style: String::from("high-top"),
            },
            Shoe{
                size: 10,
                style: String::from("mid-top"),
            },
            Shoe{
                size: 12,
                style: String::from("low-top"),
            },

        ];
        let reference = shoe_in_size_references(&shoes, 10);

        assert_eq!(reference, [
            &Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            &Shoe {
                size: 10,
                style: String::from("mid-top"),
            },
        ]);
        // this proves that it can still be referenced once the vector( made by reference)
        // is out of scope
        print!("Shoes Array: {shoes:?}");
    }
    #[test]
    fn filter_by_size(){
        let shoes: Vec<Shoe> = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe{
                size: 11,
                style: String::from("high-top"),
            },
            Shoe{
                size: 10,
                style: String::from("mid-top"),
            },
            Shoe{
                size: 12,
                style: String::from("low-top"),
            },

        ];
    
        let in_my_size = shoe_in_size(shoes,10);

        assert_eq!(in_my_size, [
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("mid-top"),
            },
        ]);
     
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
    fn iterator_mapping() {
        let v1 = vec![1, 2, 3];

        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        let first_num = v1.first().unwrap_or_else(|| &0);

        println!("{}", first_num);

        assert_eq!(v2, vec![2, 3, 4]);
    }
}