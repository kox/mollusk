#!/bin/bash

solana program dump TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA ./src/elf/token.so -u mainnet-beta
solana program dump TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb ./src/elf/token_2022.so -u mainnet-beta
solana program dump ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL ./src/elf/associated_token.so -u mainnet-beta
solana program dump CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d ./src/elf/mpl_core.so -u mainnet-beta

slot=$(solana slot -u mainnet-beta)
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' "s|//! Last updated at mainnet-beta slot height: .*|//! Last updated at mainnet-beta slot height: $slot|" ./src/lib.rs
else
    # Linux
    sed -i "s|//! Last updated at mainnet-beta slot height: .*|//! Last updated at mainnet-beta slot height: $slot|" ./src/lib.rs
fi

