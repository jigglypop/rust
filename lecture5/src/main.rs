
struct Rectangle {
    width: usize,
    height: usize
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn vercel(&self, z: usize) -> usize {
        self.area() * z
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 30
    };

    println!("사각형의 면적 : {}", rect.area());
    println!("사각형의 부피 : {}", rect.vercel(10));
}
