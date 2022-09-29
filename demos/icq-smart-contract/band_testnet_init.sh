## This script helps to initialize accounts on the testnet of the band for test purposes.

usage() {
  echo "usage: $0 [-h | --use-faucet]"
  exit
}

while [ "$1" ];
do
  arg="$1"
  case $arg in
    --use-faucet)
      shift
      echo use_faucet
      use_faucet=true
      ;;
    -h)
      shift
      usage
      ;;
    *)
      shift
      echo "error: unknown argument"
      usage
      ;;
  esac
done

# Configure variables
BINARY=bandd
HOME_BAND=$HOME/.band
CHAIN_ID=band
ALICE="use struggle faith reason method camp stay hair rabbit click across stadium there intact catalog segment drill summer oak tell tell success giraffe direct"
BOB="universe include catalog auction allow digital purity glimpse trash desert mom sea cry exchange question weekend post rival mutual scale staff law modify flee"
RELAYER_ACC="machine danger crush duck always will liberty popular security shoulder bargain day repair focus fog evoke market gossip love curious question kingdom armor crazy"

# Remove previous setup
rm -rf $HOME_BAND
 
# Bootstrap the quasar local network with single node

echo $ALICE       | $BINARY keys add alice       --keyring-backend test --recover
echo $BOB         | $BINARY keys add bob         --keyring-backend test --recover
echo $RELAYER_ACC | $BINARY keys add relayer_acc --keyring-backend test --recover

if [ "$use_faucet" = true ]
then
  # Get some tokens from faucet
  FAUCET_ADDR="https://laozi-testnet5.bandchain.org/faucet"
  curl -X POST -d '{"address":"'$($BINARY keys show alice       --keyring-backend test -a)'"}' $FAUCET_ADDR ; echo
  curl -X POST -d '{"address":"'$($BINARY keys show bob         --keyring-backend test -a)'"}' $FAUCET_ADDR ; echo
  curl -X POST -d '{"address":"'$($BINARY keys show relayer_acc --keyring-backend test -a)'"}' $FAUCET_ADDR ; echo
fi

# Check balances
RPC_ADDR="https://rpc.laozi-testnet5.bandchain.org:443"
$BINARY q bank balances $($BINARY keys show alice       --keyring-backend test -a) --node $RPC_ADDR
$BINARY q bank balances $($BINARY keys show bob         --keyring-backend test -a) --node $RPC_ADDR
$BINARY q bank balances $($BINARY keys show relayer_acc --keyring-backend test -a) --node $RPC_ADDR
