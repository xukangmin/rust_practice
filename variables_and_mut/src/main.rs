fn main() {

    const MAX_VALUE:u32 = 10;

    let x = 5;

    let x = x + 1;

    let x = MAX_VALUE - x;

    println!("x={}",x);

    let spaces = "   ";

    let spaces = spaces.len();
    
    println!("len={}",spaces);

    let k = 2.0;

    let heart_eyed_cat = '😻';

    println!("heart_eyed_cat={}",heart_eyed_cat);

    another_function(5,6);


    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!{"y={}",y};

    let x = five();

    println!("The value of x is: {}", x);
}


fn another_function(x: i32, y: i32) {
    println!{"Antoher fun={}",x};
}

fn five() -> i32 {

    let aa = 1.1231;

    aa as i32

}