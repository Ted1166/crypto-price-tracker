# CRYPTO PRICE TRACKER

The Crypto Price Tracker is a sophisticated applications designed to provide real-tie and historical price data for a wide range of cryptocurrencies. Cartesi will handle off-chain computation, making the app more scalable and efficient.

## Key Components

- Data Fetching:

   Use an API service to fetch real-time cryptocurrency prices.

   Implement the data-fetching logic within Cartesi’s off-chain environment, leveraging Linux tools and libraries.

- Smart Contract:

   Develop smart contracts to manage user accounts, preferences, and price alerts

- Backend With Cartesi:

   Utilize Cartesi Machine to handle complex computations, such as aggregating data from multiple APIs, processing alerts, and more.

- Frontend Interface:

   Create a web interface using a framework where users can view cryptocurrency prices and manage alerts.

   The frontend will interact with both the smart contracts and the off-chain Cartesi backend.

## Tool and Libraries

Rust: For writing smart contract.

Cartesi SDK: For setting up the off-chain environment and intergrating with blockchain.

CoinGecko: To fetch up the off-chain environment and intergrate with clockchain.

React: For frontend interface.