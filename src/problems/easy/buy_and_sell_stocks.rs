pub fn solve() -> i32 {
    let prices = vec![7, 1, 5, 3, 6, 4];

    // this was my initial naive solve
    // let mut highest_dif = 0;
    // for i in 0..prices.len() {
    //     for j in i..prices.len() {
    //         let highest = prices.get(j).unwrap();
    //         let lowest = prices.get(j).unwrap();
    //         if highest - lowest > highest_dif {
    //             println!("new highest range: {lowest} - {highest}");
    //             highest_dif = highest - lowest;
    //         }
    //     }
    // }
    // highest_dif

    // this is a much more efficient approach i learned online
    let mut profit = 0;
    let mut buy = prices.first().unwrap();

    for i in 0..prices.len() {
        let curr_price = prices.get(i).unwrap();
        if curr_price < buy {
            buy = curr_price;
        } else if curr_price - buy > profit {
            profit = curr_price - buy;
        }
    }
    profit
}
