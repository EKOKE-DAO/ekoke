# Additional commands related to deploy

## Add to neuron

```sh
dfx sns prepare-canisters add-nns-root --network=ic $CANISTER_PRINCIPAL
```

## Stake to neuron

```sh
quill --pem-file ekoke.pem neuron-stake --name ekoke --amount $ICP_AMOUNT > /tmp/call.txt
quill --pem-file ekoke.pem send /tmp/call.txt

quill --pem-file ekoke.pem get-neuron-info 9045316130656717892
```

## Increase dissolve delay

```sh
SECONDS_TO_ADD=15742800
quill --pem-file ekoke.pem neuron-manage -a $SECONDS_TO_ADD 9045316130656717892 > /tmp/call.txt
quill --pem-file ekoke.pem send /tmp/call.txt
```

## Propose SNS

```sh
dfx sns propose --network ic --neuron-id 9045316130656717892 sns_init.yaml
```
