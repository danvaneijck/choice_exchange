# Send to Injective Burn Auction Contract

This repository contains a CosmWasm smart contract that facilitates sending tokens (both native and CW20) to the Injective burn auction subaccount. The contract provides an interface for handling native token transfers and CW20 token operations, ensuring proper routing to the designated burn auction subaccount.

The contract requires the contract address of the INJ CW20 Adapter contract.
Source code: https://github.com/InjectiveLabs/cw20-adapter

## Features

1. **Native Token Transfers**:
   - Accepts native tokens and routes them to the Injective burn auction subaccount.
   - Ensures proper validation of funds.

2. **CW20 Token Handling**:
   - Accepts `send` messages from CW20 contracts.
   - Converts CW20 tokens into a token factory denomination and sends them to the burn auction.

3. **Admin Management**:
   - Allows updating the admin of the contract via an admin-only operation.

4. **Configurable**:
   - The contract's configuration includes the admin address, the CW20 adapter contract, and the burn auction subaccount.

---

## Messages

### InstantiateMsg
Used to initialize the contract during deployment.

```json
{
  "admin": "injective_address_of_admin",
  "adapter_contract": "injective_address_of_cw20_adapter",
  "burn_auction_subaccount": "0x1111111111111111111111111111111111111111111111111111111111111111"
}
```

- admin: The initial admin address for managing the contract.
- adapter_contract: The address of the CW20 adapter contract.
- burn_auction_subaccount: The subaccount ID for the Injective burn auction.

The burn action sub address is:
`0x1111111111111111111111111111111111111111111111111111111111111111`

### ExecuteMsg
The main entry point for executing contract actions.

`SendNative`
Sends native tokens to the burn auction.

```json
{
  "send_native": {
    "asset": {
      "info": {
        "native_token": {
          "denom": "denomination"
        }
      },
      "amount": "amount_in_wei"
    }
  }
}
```


`Receive`
Handles CW20 tokens sent via the send message from a CW20 contract

```json
{
  "receive": {
    "sender": "cw20_sender_address",
    "amount": "amount",
    "msg": "{}"
  }
}
```