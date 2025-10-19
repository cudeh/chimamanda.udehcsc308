fn main() {
<<<<<<< HEAD
    let bill_amount: f64 = 12000.0; 
    let mut discount: f64 = 0.0;    

    if bill_amount > 10000.0 {
        discount = 0.15;
    } else if bill_amount > 5000.0 {
        discount = 0.10;
    }

    let discount_amount = bill_amount * discount;
    let final_amount = bill_amount - discount_amount;

    println!("Original Bill: ₦{}", bill_amount);
    println!("Discount applied: {}%", discount * 100.0);
    println!("Final Bill after discount: ₦{}", final_amount);
=======
    // classwork 1
    println!("Hello, world!");

    let celsius = 38.7;
    let converted = (celsius * 9.0/5.0) + 32.0;

    println!("previous temperature in celsius is: {} " , celsius);
    println!("after converting to fahrenheit: {}", converted);

>>>>>>> ebd962a (My second commit)
}