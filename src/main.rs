use std::io;
use rand::Rng;
use thousands::Separable;
fn main() {
    //calc_lot();
    simurate();
}

fn input(msg: &str) -> String{
    let mut s = String::new();
    println!("{}",msg);
    io::stdin().read_line(&mut s).expect("failed to read line.");
    s.trim().parse().ok().unwrap()
}

fn calc_lot(){
    let amount: i128 = 100000;
    //証拠金維持率(%)
    let margin_rate: i128 = 300;
    let exchange_rate: f64 = 115.0;
    //0.01ロット買うのに必要な証拠金
    //0.01*10万通貨単位/レバレッジ*為替レート
    //例）0.01*100000通貨/888*116
    //通貨単位、レバレッジは固定、受け付ける入力は投資金額、現在の為替レート、維持率。出力はロット数。
}


fn simurate(){
    //let investment_amount: i128 = input("投資金額:").parse().unwrap();
    //let take_profit_pips : i128 = input("利確pips:").parse().unwrap();
    //let loss_cut_pips: i128  = input("損切pips:").parse().unwrap();
    //let purchased_lots: f64  = input("購入lots:").parse().unwrap();
    //let simurate_date_count: i128  = input("シミュレート回数").parse().unwrap();
    const WIN_RATE:i128 = 41;
    let investment_amount: i128 = 210000;
    //let take_profit_pips: i128 = 200;
    let loss_cut_pips: i128 = 50;
    let simurate_date_count: i128 = 20;
    let simurate_count: i32 = 12;
    //println!("investment amount: {}, T/P: {}pips, S/L: {}pips, simurate count: {}",investment_amount.separate_with_commas(),take_profit_pips,loss_cut_pips, simurate_date_count);
    for i in 0..simurate_count{
        let mut rng = rand::thread_rng();
        let mut amount: i128 = investment_amount;
        let mut win_count: i128 = 0;
        let mut lose_count: i128 = 0;
        let mut win_amount: i128 = 0;
        let mut lose_amount: i128 = 0;
        let mut counter: i32 = 0;
        for j in 0..simurate_date_count{
            counter += 1;
            //0.01lot = 120
            //0.01x * 120 = investment_amount / 3
            //x = investment_amount / 3 / 120 / 100
            let purchased_lots: f64 = (amount as f64 / 36000.0 * 100.0).round() / 100.0;
            println!("---------------------------------");
            println!("Day {}", j+1);
            println!("purchased lots:{}lots", purchased_lots);
            if purchased_lots < 0.01{
                println!("YOU DIED.");
                break;
            }
            let n: i128 = rng.gen_range(0..101);
            let take_profit_pips:i128 = rng.gen_range(100..201);
            //println!("random num: {}", n);
            let mut trade_result:f64 = 0.0;
            let mut result_msg = String::new();
            let mut result_pips:i128 = 0;
            if n > (100-WIN_RATE) {
                result_pips = take_profit_pips;
                trade_result = purchased_lots * take_profit_pips as f64 * 100.0;
                result_msg = String::from("win");
                win_count += 1;
                win_amount += trade_result as i128;
            } else {
                result_pips = loss_cut_pips;
                trade_result = purchased_lots * loss_cut_pips as f64 * -100.0;
                result_msg = String::from("lose");
                lose_count += 1;
                lose_amount += trade_result as i128;
            }
            amount +=  trade_result as i128;
            println!("{} {}({}pips)\ntotal amount {}",result_msg, trade_result.round().separate_with_commas(), result_pips,amount.separate_with_commas());
            if amount <= 0 {
                println!("YOU DIED.");
                break;
            }
        }
        println!("==================================================");
        println!("TOTAL RESULTS:");
        println!("total count: {}", counter.separate_with_commas());
        println!("win count: {}", win_count.separate_with_commas());
        println!("win amount: {}", win_amount.separate_with_commas());
        println!("lose count: {}", lose_count.separate_with_commas());
        println!("lose amount: {}", lose_amount.separate_with_commas());
        println!("finally profit: {}", (amount - investment_amount).separate_with_commas());
        println!("finally amount: {}", amount.separate_with_commas());
        println!("==================================================");
    }
}