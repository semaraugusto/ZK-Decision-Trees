circom circuit/decision_trees.circom --r1cs --wasm --sym

pushd decision_trees_js

node generate_witness.js decison_trees.wasm ../model_params/iris-tree-params.json witness.wtns

popd

snarkjs plonk setup decision_trees.r1cs pot12_final.ptau circuit_final.zkey
