# Choice Router <!-- omit in toc -->

The Router Contract contains the logic to facilitate multi-hop swap operations via choice exchange.

## Operations Assertion

The contract will check whether the resulting token is swapped into one token.

### Example

Swap Luna => DELIGHT => TNT

```json
{
   "execute_swap_operations":{
      "operations":[
         {
            "terra_swap":{
               "offer_asset_info":{
                  "native_token":{
                     "denom":"inj"
                  }
               },
               "ask_asset_info":{
                  "token":{
                     "contract_addr":"terra1cl0kw9axzpzkw58snj6cy0hfp0xp8xh9tudpw2exvzuupn3fafwqqhjc24"
                  }
               }
            }
         },
         {
            "terra_swap":{
               "offer_asset_info":{
                  "token":{
                     "contract_addr":"terra1cl0kw9axzpzkw58snj6cy0hfp0xp8xh9tudpw2exvzuupn3fafwqqhjc24"
                  }
               },
               "ask_asset_info":{
                  "token":{
                     "contract_addr":"terra1qnypzwqa03h8vqs0sxjp8hxw0xy5zfwyax26jgnl5k4lw92tjw0scdkrzm"
                  }
               }
            }
         }
      ],
      "minimum_receive":"1"
   }
}
```
