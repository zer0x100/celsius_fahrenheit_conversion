use std::io;

fn main() {

    println!("摂氏と華氏の書き換えを行えます。以下のいずれかの入力を行なってください。");
    println!("摂氏を華氏に直す場合はc、華氏を摂氏に直す場合fを入力しEnterキーを押した後、温度を小数点表示で入力してください。");

    let mut typed = String::new();

    //c or fの読み込み。
    io::stdin().read_line(&mut typed)
        .expect("Failed to read type");

    let typed = typed.trim();

    if (typed != "c") & (typed != "f") {
        println!("cかfを入力してください。");
    }else{
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature)
            .expect("Failed to read temperature");

            let temperature : f64 = temperature.trim().parse()
                .expect("Input a float number");

        if typed == "c" {

            let temperature: f64 = 32.0 + ( 9.0/5.0 ) * temperature;

            println!("F {}",temperature);
        }else {

            let temperature = ( 5.0/9.0 ) * (temperature - 32.0);
            
            println!("C {}",temperature);
        }
    }


}
