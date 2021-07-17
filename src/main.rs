use anyhow::{Context, Result};
use std::env;

fn parse_args(args: Vec<String>) -> Result<(i128, i128)> {
    let init: &i128 = &args[1].parse().context("cant parse arg[1]!")?;
    let times: &i128 = &args[2].parse().context("cant parse arg[2]!")?;
    Ok((*init, *times))
}

use anyhow::bail;
fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        bail!("Two numeric args are required!");
    };

    let (init, times) = parse_args(args)?;

    let mut result = init;
    for _time in 1..times {
        result = result * 2;
    }
    println!("{}", result);
    Ok(())
}
