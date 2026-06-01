struct Rectangle {
    width: i32,
    height: i32,
}

// impl Rectangle {
//     fn area (&self) -> i32{
//         return self.width * self.height;
//     }

//     fn  is_square (&self)-> bool{
//         if(self.width == self.height){
//             return true;
//         }else{
//             return false;
//         }
//     }
// }

impl Rectangle {
    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> i32 {
        return { &self.width * &self.height };
    }
}

pub fn rect_area(w: i32, h: i32) -> i32 {
    let rect = Rectangle {
        width: w,
        height: h,
    };

    // let area = rect.area();
    // let issquare = rect.is_square();

    return rect.width * rect.height;
}
