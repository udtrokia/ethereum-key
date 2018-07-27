## Ethereum Key
> A wrapper of Parity's ethkey.

### USAGE

```rust
extern crate ethereum_key;
use ethereum_key::Band;

// Keypair && Phrase
let band: Band = Band::generate();

println!("phrase:  {:?}", &band.phrase);
println!("secret:  {:?}", &band.secret);
println!("public:  {:?}", &band.public);
println!("address: {:?}", &band.address);

// Generate from phrase of secret
// let phrase: String = "hemstitch remover province donated outing oversized playoff outshoot trowel wimp palm flashily"
// let secret: String = "26d1ec50b4e62c1d1a40d16e7cacc6a6580757d5"
// let band: Band = Band::from("phranse...phranse_12");
// let band: Band = Band::from("sercret string")

```


### API
+ https://etherscan.io/apis
+ https://www.blockcypher.com/dev/ethereum/#introduction
+ https://www.etherest.io/
+ https://github.com/EverexIO/Ethplorer/wiki/ethplorer-api
+ https://www.myetherapi.com/
+ https://infura.io/docs/#calling-a-json-rpc-method

### BIPs

__BIP32__: Hierarchical Deterministic wallet, 从单一 seed 产生树状结构存储多组 keypairs (私钥和公钥)。可以方便的备份、转移到其他相容装置，以及分层权限控制。  

__BIP39__: 将 seed 方便记忆和书写的单字表示。一般由12个单字组成，称为 mnemonic code(phrase), 中文成为助记词和助记码。例如:  
```
rose rocket invest real refuse margin festival danger anger border idle brown
```  

__BIP44__: 基于 BIP32 的系统，赋予树状结构中的各层特殊的意义。让同一个 seed 可以支援多币种、多账户等。各层定义如下:  
```
m / purpose' / coin_type' / account' / change / address_index
```

__Ethereum HD wallet__:  
Ethereum 的钱包目前均采用以上 Bitcoin HD Wallet 的架构，并订 `coin_type` 为 `60'`。在一个 Ethereum HD Wallet 中，第一个账户(这里的账户是指 BIP44 中定义的 `account`) 的第一组 keypair, 其路径会是 `m/44'/60'/0'/0/0`。

Mnemonic Code Converter

### TODO:

+ [ ] wasm interface.
