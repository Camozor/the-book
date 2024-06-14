pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle<T> {
    length: T,
    height: T,
}

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
}
