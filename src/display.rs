use crate::model::PortfolioRow;
use colored::Colorize;

const COL_ID: usize = 14;
const COL_AMOUNT: usize = 12;
const COL_PRICE: usize = 14;
const COL_CHANGE: usize = 12;
const COL_VALUE: usize = 14;

fn separator() -> String {
    format!(
        "+{:-<w$}+{:-<w2$}+{:-<w3$}+{:-<w4$}+{:-<w5$}+",
        "",
        "",
        "",
        "",
        "",
        w = COL_ID + 2,
        w2 = COL_AMOUNT + 2,
        w3 = COL_PRICE + 2,
        w4 = COL_CHANGE + 2,
        w5 = COL_VALUE + 2,
    )
}

pub fn render(portfolio: &[PortfolioRow], currency: &str) {
    let cur = currency.to_uppercase();

    println!("\n{}", "  CRYPTO PORTFOLIO MONITOR".bold().cyan());
    println!("{}", separator());
    println!(
        "| {:<COL_ID$} | {:>COL_AMOUNT$} | {:>COL_PRICE$} | {:>COL_CHANGE$} | {:>COL_VALUE$} |",
        "Coin".bold(),
        "Amount".bold(),
        format!("Price ({})", cur).bold(),
        "24h %".bold(),
        format!("Value ({})", cur).bold(),
    );
    println!("{}", separator());

    let mut total = 0.0_f64;

    for row in portfolio {
        let change_str = format!("{:+.2}%", row.curr_24h_change);
        let change_colored = if row.curr_24h_change >= 0.0 {
            change_str.green()
        } else {
            change_str.red()
        };

        println!(
            "| {:<COL_ID$} | {:>COL_AMOUNT$} | {:>COL_PRICE$} | {:>COL_CHANGE$} | {:>COL_VALUE$} |",
            row.id.bold(),
            format!("{:.6}", row.amount),
            format!("{:.2}", row.price),
            change_colored,
            format!("{:.2}", row.value),
        );

        total += row.value;
    }

    println!("{}", separator());
    println!(
        "| {:<w$} | {:>COL_VALUE$} |",
        "TOTAL".bold(),
        format!("{:.2} {}", total, cur).bold().yellow(),
        w = COL_ID + 2 + COL_AMOUNT + 2 + COL_PRICE + 2 + COL_CHANGE + 3,
    );
    println!("{}\n", separator());
}
