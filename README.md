# Interface Pocs
Monorepo of POCs related to Interface RFCs

This repo is focused on solving the issue of implementing custom Managed Token programs for token transfers.

### Account Resolution

In order to find accounts needed for an instruction that dynamically executes a CPI to an unknown program, we resolve accounts needed for an instruction beforehand with a special RPC call that tells the client the list of accounts needed for a transaction.


### Digital Asset Spec Royalty 

NFT transfers require certain behavior to be followed. This differs from traditional interface definitions which leave implementation and behavior up to developers. In the case of NFTs, royalty enforcement requires certain behavioral lifecycle, but also leave some flexibility for creators to define their own royalty enforcement programs.

Original repo by Austin Adams from Metaplex Foundation here: https://github.com/metaplex-foundation/digital-asset-protocol


### View Functions

View functions can help programs conform to read interfaces.

For example, by exposing a `getRoyaltyMint` function on custom royalty enforcement programs.
