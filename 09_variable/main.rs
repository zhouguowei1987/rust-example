fn main(){
    // let Study = "";
    // println!("Study is {}", study)
    // help: a local variable with a similar name exists (notice the capitalization): `Study`

    // let price = 100;
    // price = 288;
    // println!("price is {}", price)
    // cannot assign twice to immutable variable

    let mut price = 188;
    println!("price is {}", price);
    // help: maybe it is overwritten before being read?
    price = 288;
    println!("price is {}", price)
}