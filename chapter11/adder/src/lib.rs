pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[allow(dead_code)]
fn dont_panic_bro() {
    panic!("Paniiiiiiiic");
}

#[allow(dead_code)]
struct Rectangle<T> {
    length: T,
    height: T,
}

#[allow(dead_code)]
impl<T: std::cmp::PartialOrd> Rectangle<T> {
   fn can_hold(&self, other_rectangle: &Rectangle<T>) -> bool {
       self.length >= other_rectangle.length && self.height >= other_rectangle.height
   }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn it_should_fail() {
    //     panic!("Fail");
    // }
    //
    
    #[test]
    fn test_rectangle_can_hold() {
        let rectangle1: Rectangle<f32> = Rectangle {
            length: 156.2,
            height: 145.8,
        };
        let rectangle2: Rectangle<f32> = Rectangle {
            length: 125.9,
            height: 142.8,
        };
        assert!(rectangle1.can_hold(&rectangle2));
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        dont_panic_bro();
    }
}
