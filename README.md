
#### Get ETH balance of given address aronud given timestamp 
```
./target/debug/rusty_eth -a 0x9696f59E4d72E237BE84fFD425DCaD154Bf96976 -t  'Wed, 20 Apr 2022 00:00:00 +0530'                                                                            Calling accounts.
Accounts: []
Calling balance.
Balance of 0x9696f59e4d72e237be84ffd425dcad154bf96976
     17.00    ┼
                 ETH balance during blocks 14617201 - 14617201 with sample counts 1
```

####  Get ERC20 Token balance for given token contract and for given address around given timestamp
```
./target/debug/rusty_eth -a 0xf24c609e942a65efa7f745f75c16a7a7d8d04834 --erc-20-token-address 0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984 --erc-20-token-abi uniswap_token.json -t 'Wed, 20 Apr 2022 00:00:00 +0530'
Calling accounts.
Accounts: []
Balance of 0xf24c609e942a65efa7f745f75c16a7a7d8d04834 for ERC20 Contract at 0x1f9840a85d5af5bf1d1762f925bdaddc4201f984
      7.00   ┼
                Token balance during blocks 14617201 - 14617201 with sample counts 1
```

####  Get ERC20 Token balance for given token contract and for given address during given blocknumber range
```
./target/debug/rusty_eth -a 0xf24c609e942a65efa7f745f75c16a7a7d8d04834 -s 12496385 -e 14629255 --erc-20-token-address 0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984 --erc-20-token-abi uniswap_token.json 
Calling accounts.
Accounts: []
Balance of 0xf24c609e942a65efa7f745f75c16a7a7d8d04834 for ERC20 Contract at 0x1f9840a85d5af5bf1d1762f925bdaddc4201f984
     17.95    ┼                  ╭╮ 
     16.16    ┤                  ││ 
     14.36    ┤                  ││ 
     12.57    ┤               ╭╮ ││ 
     10.77    ┤               │╰╮││ 
      8.98    ┤               │ │││ 
      7.18    ┤               │ ││╰ 
      5.39    ┤               │ ╰╯  
      3.59    ┤             ╭─╯     
      1.80    ┤             │       
      0.00    ┼─────────────╯      
                 Token balance during blocks 12496385 - 14629245 with sample counts 20
```

#### Get ETH balance of given address during given blocknumber range
```
./target/debug/rusty_eth -a 0xf24c609e942a65efa7f745f75c16a7a7d8d04834 -s 12496385 -e 14629255
Calling accounts.
Accounts: []
Calling balance.
Balance of 0xf24c609e942a65efa7f745f75c16a7a7d8d04834
     13.89    ┼  ╭╮                 
     12.50    ┤  ││           ╭╮    
     11.12    ┤  ││       ╭╮  ││    
      9.73    ┤  ││       │╰╮ │╰╮   
      8.34    ┤  ││       │ │ │ ╰╮  
      6.96    ┤  ││       │ ╰╮│  │  
      5.57    ┤  ││       │  ││  │  
      4.18    ┤  ││       │  ││  ╰─ 
      2.80    ┤  ││       │  ││     
      1.41    ┤  ││       │  ││     
      0.02    ┼──╯╰───────╯  ╰╯    
                 ETH balance during blocks 12496385 - 14629245 with sample counts 20
```
