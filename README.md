## REST-Like API for Ethereum

### API
+ https:://etherscan.io/apis
+ https://www.blockcypher.com/dev/ethereum/#introduction
+ https://www.etherest.io/
+ https://github.com/EverexIO/Ethplorer/wiki/ethplorer-api
+ https://www.myetherapi.com/
+ https://infura.io/docs/#calling-a-json-rpc-method

### BIPs

BIP32: Hierarchical Deterministic wallet, 从单一 seed 产生树状结构存储多组 keypairs (私钥和公钥)。可以方便的备份、转移到其他相容装置，以及分层权限控制。  

BIP39: 将 seed 方便记忆和书写的单字表示。一般由12个单字组成，称为 mnemonic code(phrase), 中文成为助记词和助记码。例如:  
```
rose rocket invest real refuse margin festival danger anger border idle brown
```  

BIP44: 基于 BIP32 的系统，赋予树状结构中的各层特殊的意义。让同一个 seed 可以支援多币种、多账户等。各层定义如下:  
```
m / purpose' / coin_type' / account' / change / address_index
```

Ethereum HD wallet:  
Ethereum 的钱包目前均采用以上 Bitcoin HD Wallet 的架构，并订 `coin_type` 为 `60'`。在一个 Ethereum HD Wallet 中，第一个账户(这里的账户是指 BIP44 中定义的 `account`) 的第一组 keypair, 其路径会是 `m/44'/60'/0'/0/0`。

Mnemonic Code Converter
