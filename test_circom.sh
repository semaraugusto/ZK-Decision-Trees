circuit_path=$1
circuit_name="$(basename $circuit_path .circom)"
input_path=../$2

circom $circuit_path --r1cs --wasm --sym

cd $circuit_name\_js

node generate_witness.js $circuit_name.wasm $input_path witness.wtns

cd ../

# snarkjs plonk setup $circuit_name.r1cs pot12_final.ptau circuit_final.zkey
