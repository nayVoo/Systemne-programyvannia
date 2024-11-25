fn bon_appetit(bill: Vec<i32>, k: usize, b: i32) {
    let total_cost: i32 = bill.iter().sum();
    let anna_share = (total_cost - bill[k]) / 2;

    if b == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_share);
    }
}

fn main() {
    let bill = vec![3, 10, 2, 9];
    let k = 1;
    let b = 12;
    bon_appetit(bill, k, b);

    let bill = vec![3, 10, 2, 9];
    let k = 1;
    let b = 7;
    bon_appetit(bill, k, b);
}
