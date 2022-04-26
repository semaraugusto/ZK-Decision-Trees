
pragma circom 2.0.0;

include "../node_modules/circomlib/circuits/comparators.circom";

template DecisionTreeParser(num_nodes, num_features) {
    signal input features[num_nodes];
    signal input thresholds[num_nodes];
    signal input in[num_features];
    signal output out;
    /* var bignumb <== 10000000; */
    component is_equal = IsEqual();
    log(features[0]);
    log(thresholds[0]);
    is_equal.in[0] <== features[0];
    is_equal.in[1] <== thresholds[0];
    is_equal.out === 0;
    out <== is_equal.out;
}

component main = DecisionTreeParser(19, 4);
