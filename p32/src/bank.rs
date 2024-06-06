#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub credit_line: u64,
    pub balance: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bank {
    pub users: Vec<User>,
    pub name: String,
    pub credit_interest: u64,
    pub debit_interest: u64,
}

impl User {
    pub fn new(name: String, credit_line: u64, balance: i64) -> Self {
        Self {
            name,
            credit_line,
            balance,
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
    pub fn set_credit_line(&mut self, credit_line: u64) {
        self.credit_line = credit_line
    }
    pub fn set_balance(&mut self, balance: i64) {
        self.balance = balance
    }
    pub fn get_name(&mut self) -> String {
        self.name.clone()
    }
    pub fn get_credit_line(&self) -> u64 {
        self.credit_line
    }
    pub fn get_balance(&self) -> i64 {
        self.balance
    }
}

impl Bank {
    pub fn new(users: Vec<User>, name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Self {
            users,
            name,
            credit_interest,
            debit_interest,
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
    pub fn set_credit_interest(&mut self, credit_interest: u64) {
        self.credit_interest = credit_interest
    }
    pub fn set_debit_interes(&mut self, debit_interest: u64) {
        self.debit_interest = debit_interest
    }
    pub fn set_users(&mut self, users: Vec<User>) {
        self.users = users
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_credit_interest(&self) -> u64 {
        self.credit_interest
    }
    pub fn get_debit_interes(&self) -> u64 {
        self.debit_interest
    }
    pub fn get_users(&self) -> Vec<User> {
        self.users.clone()
    }
    fn add_user(&mut self, user: User) {
        self.users.push(user)
    }
    fn find_user_by_name(&self, name: String) -> Option<usize> {
        self.users.iter().position(|x| x.name == name)
    }
    pub fn calc_balance(&self) -> (i64, i64) {
        let mut liabilities: i64 = 0;
        let mut assets: i64 = 0;
        for user in self.users.iter() {
            if user.balance > 0 {
                liabilities += user.balance;
            } else {
                assets += user.balance;
            }
        }
        (liabilities, assets)
    }
    pub fn transfer_funds(&mut self, sender: String, receiver: String, mount: u64) {
        let pos_sender = self.find_user_by_name(sender);
        let pos_receiver = self.find_user_by_name(receiver);

        if pos_sender.is_some() && pos_receiver.is_some() {
            let credit = self.users[pos_sender.unwrap()].credit_line;
            if credit.checked_sub(mount).is_some() {
                let mut balance = self.users[pos_sender.unwrap()].balance;
                self.users[pos_sender.unwrap()].set_balance(balance - (mount as i64));
                balance = self.users[pos_receiver.unwrap()].balance;
                self.users[pos_receiver.unwrap()].set_balance(balance + (mount as i64));
                println!("Successful transaction!!")
            } else {
                println!("Transaction fails:: insufucuent funds!!")
            }
        } else {
            println!("Transaction fails: user not found!!")
        }
    }
    pub fn accrue_interest(&mut self) {
        let users = self.users.iter_mut();
        for user in users {
            if user.balance > 0 {
                let rates = (((self.debit_interest + 100) as i64) * user.balance) / 100;
                user.set_balance(rates);
            } else {
                let rates = (((self.credit_interest + 100) as i64) * user.balance) / 100;
                user.set_balance(rates);
            }
        }
    }
    pub fn merge_bank(&mut self, merged_bank: Bank) {
        let new_users = merged_bank.users;
        for user in new_users {
            let pos = self.find_user_by_name(user.name.clone());
            if pos.is_some() {
                let balance = self.users[pos.unwrap()].balance;
                self.users[pos.unwrap()].set_balance(user.balance + balance);
            } else {
                self.add_user(user);
            }
        }
    }
}
