extern crate rasciigraph;
use chrono::DateTime;
use clap::Parser;
use rasciigraph::{plot, Config};
use std::str::FromStr;
use web3::{
    contract::{Contract, Options},
    futures::future::try_join_all,
    types::{BlockNumber, U256},
    Transport,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // address of interest
    #[clap(short, long, default_value_t = String::from("f24c609e942a65efa7f745f75c16a7a7d8d04834"))]
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
    // timestamp in RFC2822 format at which balance is reuqired to be found
    #[clap(short, long)]
    timestamp: Option<String>,
}

//TODO: Add unit tests for this function.
// This function does not return block_number with high accuracy.
async fn timestamp_to_block<T: Transport>(web3: web3::Web3<T>, timestamp: String) -> u64 {
    let timestamp: u64 = DateTime::parse_from_rfc2822(&timestamp)
        .expect("timestamp parsing failed")
        .timestamp()
        .try_into()
        .expect("error creating timestamp");
    let timestamp: U256 = timestamp.into();

    let average_block_time = U256::from(14_u64);
    let mut current_block_number = web3
        .eth()
        .block_number()
        .await
        .expect("get current block number failed");
    let mut current_block = web3
        .eth()
        .block(current_block_number.into())
        .await
        .expect("get current block failed")
        .unwrap();
    while current_block.timestamp > timestamp.into() {
        let decrease_block = (current_block.timestamp - timestamp) / average_block_time;
        let decrease_block_int = decrease_block.low_u64() as u64;
        if decrease_block_int < 1_u64 {
            break;
        }
        current_block_number -= decrease_block_int.into();
        current_block = web3
            .eth()
            .block(current_block_number.into())
            .await
            .expect("get current block failed")
            .unwrap();
    }

    current_block_number.low_u64().into()
}

#[tokio::main]
//TODO: breakdown main's code into small functions
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
    let mut sample_counts = 20;
    let mut start_block = args.start_block_number;
    let mut end_block = args.end_block_number;
    if let Some(timestamp) = args.timestamp {
        start_block = timestamp_to_block(web3.clone(), timestamp).await;
        end_block = start_block;
        sample_counts = 1; // just for printing purpose
    }
    let step = (end_block - start_block) as f64 / sample_counts as f64;
    let mut step = step.floor() as u64;
    if step < 1 {
        step = 1;
    }
    // get balance of ETH
    if args.erc_20_token_address.is_none() {
        println!("Calling balance.");
        for account in accounts {
            let mut futures = Vec::new();
            let mut blocks = Vec::new();
            for block in (start_block..=end_block).step_by(step as usize) {
                let balance = web3
                    .eth()
                    .balance(account, Some(BlockNumber::Number(block.into())));
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
        // get balance of ERC20 tokens
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
            for block in (start_block..=end_block).step_by(step as usize) {
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
