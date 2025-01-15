fn main(){
    let lang = "rust";
    println!("hello {lang}");
    println!("hello {} {}", lang, lang);

    let x = 2;
    println!("{0} * {0} = {1}", x, x * x);

    let y = 5;
    let z = 10;
    println!("{0} * {1} = {2}", y,z, y *z);
}