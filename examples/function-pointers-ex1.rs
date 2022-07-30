fn afunc(x: &str, y: u8)  -> String {
   format!("{}-{}", x, y)
}

type LabelMaker = fn(&str, u8) -> String; 

fn main() {
    
    let ptr_afunc = &afunc ;

    let make_label: LabelMaker = afunc;
    
    let another = afunc; 

    println!("{}", ptr_afunc("RX", 100));
    println!("{}", make_label("Bullet", 35));
    println!("{}", another("FZ", 16));
}