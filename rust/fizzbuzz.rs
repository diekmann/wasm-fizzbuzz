fn fizzbuzz(n: i32){
    if n % 15 == 0 {
        println!("fizzbuzz");
        return
    }
    if n % 5 == 0 {
        println!("buzz");
        return
    }
    if n % 3 == 0 {
        println!("fizz");
        return
    }

    println!("{}", n);
}

fn main(){
    for n in 0..100 {
        fizzbuzz(n);
    }
}
