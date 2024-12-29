use std::io;

pub fn convertFtC(){
    println!("enter your temp F");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");

    let tempnum = temp.trim().parse::<f64>().unwrap();
    let converted: f64 = (5.0/9.0)*(tempnum as f64 - 32.0) ;
    println!("your temp is {:.2} C", converted);
}

pub fn converCtF(){
    println!("enter your temp C");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let tempnum = temp.trim().parse::<f64>().unwrap();
    let converted: f64 = (tempnum * (9.0/5.0)) + 32.0 ;
    println!("your temp is {:.1} F", converted);

}