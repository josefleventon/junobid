# Contract deploy script
# Run it like this: `zsh ./scripts/deploy_testnet.sh`

# View your keys with `junod keys list`

export CONTRACT_NAME=juno_bid;
export KEY_NAME=testnet;

export WALLET_DATA=$(junod keys show $KEY_NAME --output json | jq .);

export KEY_NAME=$(echo $WALLET_DATA | jq -r '.name');
export KEY_TYPE=$(echo $WALLET_DATA | jq -r '.type');
export KEY_ADDRESS=$(echo $WALLET_DATA | jq -r '.address');

echo "\nConnected to wallet '$KEY_NAME'<$KEY_TYPE> @ $KEY_ADDRESS";
echo "\n========\n";

# Instantiate message config
export INSTANTIATE_MSG="{\"admins\": [\"$KEY_ADDRESS\"]}";

## INIT ##
# Get network config
echo "Sourcing network configuration...";

export CHAIN_ID="uni-5";
export FEE_DENOM="junox";
export STAKE_DENOM="junox";
export BECH32_HRP="juno";

export RPC="https://rpc.uni.junonetwork.io:443";

# Tx flag configuration
export NODE=(--node $RPC);
export TXFLAG=($NODE --chain-id $CHAIN_ID --gas-prices 0.25ujunox --gas auto --gas-adjustment 1.3);

echo "Network configuration found."

## BUILD ##
# If the architecture is `arm64`, run the arm64 version of rust-optimizer
echo "\n========\n";
echo "Building contract...";

export ARCH='';
export L_ARCH='';

if [[ $(uname -m) -eq 'arm64' ]]
then
  ARCH='-arm64';
  LARCH='-aarch64';
fi

docker run --rm -v "$(pwd)":/code \
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/rust-optimizer$ARCH:0.12.8;

CONTRACT_NAME=$CONTRACT_NAME$LARCH;

## DEPLOY ##
# Fetch codeids
echo "\n========\n";
echo "Fetching CodeIDs...";
export RES=$(junod tx wasm store artifacts/$CONTRACT_NAME.wasm --from $KEY_NAME $TXFLAG -y --output json -b block);
export CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value');
echo "CodeID found: $CODE_ID";

# Instantiate the contract
echo "\n========\n";
echo "Instantiating contract...";
junod tx wasm instantiate $CODE_ID "$INSTANTIATE_MSG" --from $KEY_NAME --label "$CONTRACT_NAME" $TXFLAG -y --no-admin;
echo "Contract instantiated."

# Store contract addr in $CONTRACT
echo "\n========\n";
echo "Fetching contract address...";
sleep 6;
export CONTRACT=$(junod query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]');
echo "Contract address: $fg_bold[green]$CONTRACT";