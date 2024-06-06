use bank::*;

pub mod bank;
fn main() {
    println!("\n\nBank model 1\n");

    let user1 = User::new(String::from("Eduardo"), 100, -30);
    let user2 = User::new(String::from("Jose"), 100, 40);
    println!("{:?} {:?}", user1, user2);

    let mut users_list: Vec<User> = Vec::with_capacity(10);
    users_list.push(user1);
    users_list.push(user2);

    let mut bank: Bank = Bank::new(users_list, String::from("My_Bank"), 10, 10);
    println!("{:?}", bank);

    println!("\n\nBank model 2\n");

    let (liabilities, assets) = bank.calc_balance();
    println!(
        "The {}'s balance is: [liabilities = {}, assets = {}]\n",
        bank.name, liabilities, assets
    );

    bank.transfer_funds(String::from("Eduardo"), String::from("Jose"), 10);
    println!("{:?}\n", bank);

    bank.accrue_interest();
    println!("{:?}\n", bank);

    println!("\n\nBank model 3\n");

    let user1 = User::new(String::from("Eduardo"), 100, -30);
    let user3 = User::new(String::from("Luis"), 100, 50);
    let user4 = User::new(String::from("Leslie"), 100, -20);
    println!("{:?} {:?} {:?}", user3, user4, user1);

    let mut users_list: Vec<User> = Vec::with_capacity(10);
    users_list.push(user1);
    users_list.push(user3);
    users_list.push(user4);

    let mut bank2: Bank = Bank::new(users_list, String::from("My_Bank_new"), 20, 20);
    println!("{:?}", bank2);

    bank2.merge_bank(bank);
    println!("\n{:?}", bank2);
    // println!("\n{:?}", bank1);
}
