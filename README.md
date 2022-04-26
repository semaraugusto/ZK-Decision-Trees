# ZK-Decision-Trees

A circom  implementation of decision trees prediction algorithm. Can be used to do verifiable machine learning

Unfortunately one cannot use model parameters as an input to the circom circuit. This is due to the fact that on a decision tree each node has both a feature identifier and a threshold value. Since signals cannot be used to index arrays inside circom, we need to find another solution.

My plan is to create a template dt implementation and create some program that parses both tree specification and program template and produces final circuit

# TODO 
* 
