struct Computers {
     hp:f32,
     ibm:f32,
     toshiba:f32,
     dell:f32
}

impl Computers {
    fn sum(&self) -> f32 {
        self.hp * 3.0 + self.ibm * 3.0 + self.toshiba * 3.0 + self. dell * 3.0
    }
}

fn main() {
    let cost = Computers {
      hp: 650000.0,
      ibm: 755000.0,
      toshiba: 550000.0,
      dell: 850000.0,
    };

    println!("The total cost of the computers is {}", cost.sum());

}
