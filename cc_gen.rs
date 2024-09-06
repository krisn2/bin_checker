use rand::prelude::*;
use chrono::{Duration, Utc};
use rand::seq::SliceRandom;

struct CC<'a> {
    cc_type: &'a str,
    cc_len: usize,
    cc_num: String,
    cc_cvv: String,
    cc_exp: String,
    cc_prefill: Vec<u32>,
}

impl<'a> CC<'a> {
    const CCDATA: &'static [(&'static str, usize, usize, &'static [u32])] = &[
        ("amex", 15, 4, &[34, 37]),
        ("discover", 16, 3, &[6001]),
        ("mc", 16, 3, &[51, 55]),
        ("visa13", 13, 3, &[4]),
        ("visa16", 16, 3, &[4]),
    ];

    fn new(cc_type: &'a str) -> Self {
        Self {
            cc_type,
            cc_len: 0,
            cc_num: String::new(),
            cc_cvv: String::new(),
            cc_exp: String::new(),
            cc_prefill: Vec::new(),
        }
    }

    fn generate_cc_exp(&mut self) {
        let start_year = Utc::now().year() + 1;
        let end_year = start_year + 2;

        let random_year = rand::thread_rng().gen_range(start_year..=end_year);
        let random_month = rand::thread_rng().gen_range(1..=12);

        self.cc_exp = format!("{:02}-{}", random_month, random_year);
    }

    fn generate_cc_cvv(&mut self) {
        let length = Self::get_ccdata(self.cc_type).1;
        self.cc_cvv = (0..length)
            .map(|_| rand::thread_rng().gen_range(0..=9).to_string())
            .collect();
    }

    fn generate_cc_prefill(&mut self) {
        let prefill_choices = Self::get_ccdata(self.cc_type).3;
        self.cc_prefill = vec![*prefill_choices.choose(&mut rand::thread_rng()).unwrap()];
    }

    fn generate_cc_num(&mut self) {
        let remaining = Self::get_ccdata(self.cc_type).0 - self.cc_prefill.len();
        let mut working: Vec<u32> = self.cc_prefill.clone();

        working.extend((0..remaining - 1).map(|_| rand::thread_rng().gen_range(1..=9)));

        let check_offset = (working.len() + 1) % 2;
        let mut check_sum = 0;

        for (i, &n) in working.iter().enumerate() {
            if (i + check_offset) % 2 == 0 {
                let n_ = n * 2;
                check_sum += if n_ > 9 { n_ - 9 } else { n_ };
            } else {
                check_sum += n;
            }
        }

        let temp = working.clone();
        self.cc_num = temp.iter().map(|x| x.to_string()).collect::<String>();
    }

    fn return_new_card(&self) -> CardDetails {
        CardDetails {
            cc_type: self.cc_type.to_string(),
            cc_num: self.cc_num.clone(),
            cc_cvv: self.cc_cvv.clone(),
            cc_exp: self.cc_exp.clone(),
        }
    }

    fn print_new_card(&self) {
        let hr = "--------------------------------";
        println!("{}", hr);
        println!("Type: {}", self.cc_type);
        println!("Number: {}", self.cc_num);
        println!("CVV: {}", self.cc_cvv);
        println!("Exp: {}", self.cc_exp);
    }

    fn get_ccdata(cc_type: &str) -> (usize, usize, usize, &'static [u32]) {
        for &(ty, len_num, len_cvv, pre) in Self::CCDATA.iter() {
            if ty == cc_type {
                return (len_num, len_cvv, 0, pre);
            }
        }
        panic!("Card type not recognized.");
    }
}

struct CCNumGen<'a> {
    card_list: Vec<CardDetails>,
    card_type: &'a str,
    num: usize,
}

struct CardDetails {
    cc_type: String,
    cc_num: String,
    cc_cvv: String,
    cc_exp: String,
}

impl<'a> CCNumGen<'a> {
    const HR: &'static str = "--------------------------------";
    const CARD_TYPES: &'static [&'static str] = &["amex", "discover", "mc", "visa13", "visa16"];

    fn new(card_type: &'a str, num: usize) -> Self {
        if !Self::CARD_TYPES.contains(&card_type) {
            panic!("Card type not recognized. Task ended.");
        }

        let mut generator = Self {
            card_list: Vec::with_capacity(num),
            card_type,
            num,
        };

        println!("{}", Self::HR);
        println!("Generating {} {} cards...", num, card_type);

        for _ in 0..num {
            let mut card = CC::new(card_type);
            card.generate_cc_exp();
            card.generate_cc_cvv();
            card.generate_cc_prefill();
            card.generate_cc_num();
            generator.card_list.push(card.return_new_card());
            card.print_new_card();
        }

        println!("{}", Self::HR);
        println!("Task complete.");
        println!("{}", Self::HR);

        generator
    }

    fn print_card_list(&self) {
        for card in &self.card_list {
            println!("{}", Self::HR);
            println!("Type: {}", card.cc_type);
            println!("Number: {}", card.cc_num);
            println!("CVV: {}", card.cc_cvv);
            println!("Exp: {}", card.cc_exp);
        }
    }
}

fn main() {
    let _amex = CCNumGen::new("amex", 2);
    let _discover = CCNumGen::new("discover", 2);
    let _mc = CCNumGen::new("mc", 2);
    let _visa13 = CCNumGen::new("visa13", 2);
    let _visa16 = CCNumGen::new("visa16", 2);
}
