#!/bin/bash

# Script chamado pelo irace para executar o programa

# Recebe como argumentos:
# $1 é o ID da configuração
# $2 é o ID da instância
# $3 é o seed (número aleatório)
# $4 é o nome da instância
# Os demais argumentos ($5 e posteriores) são os parâmetros a serem testados

CONFIG_ID=$1
INSTANCE_ID=$2
SEED=$3
INSTANCE=$4
shift 4 || exit 1
CONFIG_PARAMS=$*

# Define número de trials para cada execução
TRIALS=30

# Executa seu programa
RESULT=$(cargo run --release -- --instance $INSTANCE --trials $TRIALS $CONFIG_PARAMS)

if [ $? -ne 0 ]; then
  echo "999999999"
  exit 0
fi

# Retorna o resultado para o irace
echo "$RESULT"
