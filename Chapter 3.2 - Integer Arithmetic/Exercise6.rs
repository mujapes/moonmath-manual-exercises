fn main() {
    let mut dividend: i32 = 6_412_616;
    let divisor: i32 = 442;
    let divisor_len = divisor.ilog10() + 1;
    let mut quotient = 0;
    while dividend >= divisor {
        let dividend_len = dividend.ilog10() + 1;
        let mut next_quotient_base = 10i32.pow(dividend_len - divisor_len);
        if next_quotient_base * divisor > dividend {next_quotient_base /= 10;}
        for multiple in (1..10).rev() {
            let next_product = next_quotient_base * multiple * divisor;
            if next_product <= dividend {
                dividend -= next_product;
                quotient += next_product / divisor;
                break
            }
        }
    }
    println!("Quotient: {}, Remainder: {}", quotient, dividend);
}