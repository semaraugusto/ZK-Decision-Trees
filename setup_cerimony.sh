set -e # Exit in case of any errors
snarkjs powersoftau new bn128 18 pot18_0000.ptau -v
snarkjs powersoftau contribute pot18_0000.ptau pot18_0001.ptau --name="First contribution" -v
snarkjs powersoftau contribute pot18_0001.ptau pot18_0002.ptau --name="Second contribution" -v -e="some random text"
snarkjs powersoftau export challenge pot18_0002.ptau challenge_0003
snarkjs powersoftau challenge contribute bn128 challenge_0003 response_0003 -e="some random text"
snarkjs powersoftau import response pot18_0002.ptau response_0003 pot18_0003.ptau -n="Third contribution name"
snarkjs powersoftau beacon pot18_0003.ptau pot18_beacon.ptau 0102030405060708090a0b0c0d0e0f101118131415161718191a1b1c1d1e1f 10 -n="Final Beacon"
snarkjs powersoftau prepare phase2 pot18_beacon.ptau pot18_final.ptau -v
snarkjs powersoftau verify pot18_final.ptau
