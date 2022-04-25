extern crate rasciigraph;
use clap::Parser;
use rasciigraph::{plot, Config};
use std::str::FromStr;
use web3::{
    contract::{Contract, Options},
    futures::future::try_join_all,
    types::{BlockNumber, U256},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // address of interest
    #[clap(short, long, default_value_t = String::from("6f140f64FC13482F2209eB1cCF9c2bf633409B78"))]
    address: String,
    // starting blocknumber
    #[clap(short, long, default_value_t = 0)]
    start_block_number: u64,
    // ending blocknumber
    #[clap(short, long, default_value_t = 14619595)]
    end_block_number: u64,
    // smart contract address for ERC20 token
    #[clap(long)]
    erc_20_token_address: Option<String>,
    // path of JSON file containing ABI for ERC20 token
    #[clap(long)]
    erc_20_token_abi: Option<String>,
}

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let args = Args::parse();
    let transport = web3::transports::Http::new(
        "https://eth-mainnet.alchemyapi.io/v2/ke4CdMGOxbLwdMkbhicnFTTO5X6S-xJu",
    )?;
    let web3 = web3::Web3::new(transport);
    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push(
        web3::types::Address::from_str(&args.address).expect("string to address conversion failed"),
    );
    let sample_counts = 20;
    let step = (args.end_block_number - args.start_block_number) as f64 / sample_counts as f64;
    let mut step = step.floor() as u64;
    if step < 1 {
        step = 1;
    }
    if args.erc_20_token_address.is_none() {
        println!("Calling balance.");
        for account in accounts {
            let mut futures = Vec::new();
            let mut blocks = Vec::new();
            for block in (args.start_block_number..=args.end_block_number).step_by(step as usize) {
                let balance = web3.eth().balance(
                    account,
                    /*None*/ Some(BlockNumber::Number(block.into())),
                );
                futures.push(balance);
                blocks.push(block as f32);
            }

            let balances: Vec<U256> = try_join_all(futures).await?;
            let eth_balaces: Vec<f64> = balances
                .iter()
                .map(|b| b.low_u64() as f64 / 10_u64.pow(18) as f64)
                .collect();
            let first_x_point = *blocks.first().unwrap();
            let last_x_point = *blocks.last().unwrap();
            println!("Balance of {:?}", account);
            let caption = format!(
                "ETH balance during blocks {} - {} with sample counts {}",
                first_x_point, last_x_point, sample_counts
            );
            println!(
                "{}",
                plot(
                    eth_balaces,
                    Config::default()
                        .with_offset(10)
                        .with_height(10)
                        .with_caption(caption)
                )
            );
        }
    } else {
        if args.erc_20_token_abi.is_none() {
            println!("Please provide erc_20_token_abi");
            return Ok(());
        }
        let erc_20_token_address =
            web3::types::Address::from_str(&args.erc_20_token_address.unwrap())
                .expect("string to address conversion failed");
        let abi_file_path = args.erc_20_token_abi.unwrap();
        let abi_bytes = std::fs::read(abi_file_path).expect("Can't read ABI from file");
        let contract = Contract::from_json(web3.eth(), erc_20_token_address, &abi_bytes)?;
        for account in accounts {
            let mut futures = Vec::new();
            let mut blocks = Vec::new();
            for block in (args.start_block_number..=args.end_block_number).step_by(step as usize) {
                let balance = contract.query(
                    "balanceOf",
                    (account,),
                    None,
                    Options::default(),
                    Some(BlockNumber::Number(block.into()).into()),
                );
                futures.push(balance);
                blocks.push(block as f32);
            }

            let balances: Vec<U256> = try_join_all(futures).await?;
            let eth_balaces: Vec<f64> = balances
                .iter()
                .map(|b| b.low_u64() as f64 / 10_u64.pow(18) as f64)
                .collect();
            let first_x_point = *blocks.first().unwrap();
            let last_x_point = *blocks.last().unwrap();
            println!(
                "Balance of {:?} for ERC20 Contract at {:?}",
                account,
                contract.address()
            );
            let caption = format!(
                "Token balance during blocks {} - {} with sample counts {}",
                first_x_point, last_x_point, sample_counts
            );
            println!(
                "{}",
                plot(
                    eth_balaces,
                    Config::default()
                        .with_offset(10)
                        .with_height(10)
                        .with_caption(caption)
                )
            );
        }
    }

    Ok(())
}