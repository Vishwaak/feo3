fn main() {
    let mut count = 0;
    loop{
        count += 1;
        println!("Number: {}", count);
        if count == 20
        {
            break;
        }
    }
    count = 0;
    while count <=100
    {
        if count % 15 == 0{
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        }
        else if count % 5 == 0{
            println!("Buzz");
        }
        else
        {
            println!("{}" , count);
        }
        count += 1;
    }
    for x in 0..100 {
         if x % 15 == 0{
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        }
        else if x % 5 == 0{
            println!("Buzz");
        }
        else
        {
            println!("{}" , x);
        }
        count += 1;
    }
    
    println!("Hello, world! ");
}
