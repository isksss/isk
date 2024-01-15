use rand::Rng;
/// 乱数生成して表示
/// 実質スロット
pub fn rand(num:u32){

    // 表示
    println!("|====|====|====|");
    for _ in 0..num {
        slot();
    }
    println!("|====|====|====|");

}

fn slot(){
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let num1: u32 = rng.gen_range(0..100);
    let num2: u32 = rng.gen_range(0..100);
    let num3: u32 = rng.gen_range(0..100);

    // 表示
    println!("| {:>2} | {:>2} | {:>2} |", num1, num2, num3);

}