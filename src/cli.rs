use clap::{Parser, ValueEnum};
use std::fmt::{Formatter};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Parameters {
    // ローンの年利
    #[arg(short, long)]
    annual_interest_rate: f32,
    // 返済年数
    #[arg(short, long)]
    years: u32,
    #[arg(long)]
    debt_amount: i64,
    //  short,long を設定しない場合はキーワード引数が設定できない(宣言の順番に入れる必要がある)
    // boolの場合引数を与えるとtrue,与えないとfalseと設定可能
    #[arg(short, long)]
    pub debug: bool,
    #[arg(default_value_t = RepaymentPlan::EqualPrincipal)]
    pub repayment_plan: RepaymentPlan,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum RepaymentPlan {
    // 元金均等返済
    EqualTotal,
    // 元利均等返済
    EqualPrincipal,
}

impl std::fmt::Display for RepaymentPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            RepaymentPlan::EqualTotal => write!(f, "equal-total"),
            RepaymentPlan::EqualPrincipal => write!(f, "equal-principal")
        }
    }
}