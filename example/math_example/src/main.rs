fn main() {
    let x = 19595;
    let y = 25458;

    println!("{}", sum(x, y));
    println!("{}", i32::saturating_add(x, y));
    println!("{}", i32::wrapping_add(x, y));

}
fn sum(x:i32, y:i32) ->i32{
    x+y
}

fn sub(x:i32, y:i32) ->i32{
    x-y
}

fn mult(x:i32, y:i32) ->i32{
    x*y
}
fn div(x:i32, y:i32) ->i32{
    x/y
}


