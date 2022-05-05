# ZK-Decision-Trees

A circom  implementation of decision trees prediction algorithm. Can be used to do verifiable machine learning

Unfortunately one cannot use model parameters as an input to the circom circuit. This is due to the fact that on a decision tree each node has both a feature identifier and a threshold value. Since signals cannot be used to index arrays inside circom, we need to find another solution.
((I mean... I think with QuinSelector for very few decision trees it might actually be possible. But since decision trees usually are only used under stronger algorithms like random forests and that requires dozens or hundreds of trees thus making it impractical))

My plan is to create a template dt implementation and create some program that parses both tree specification and program template and produces final circuit

encoding for tree nodes is done using a more than succint technique that takes advantage of the fact that Decision Trees have the characteristic that either the node is a leaf node or it has two children. There's no node with a single child in a decision tree.

This allows for a decryption that works as follows

```rust
fn recursion(...) -> Option<Node>
    if feature_idxs.is_empty() {
        return None;
    }
    let val = feature_idxs[0];
    if val == -2 {
        // is leaf node
        return Some(Node { leaf }
    }
    let left = recursion(feature_idxs[1..])
    let right = recursion(feature_idxs[1..])
    return Some(Node { val, left, right });
```

# TODO 
* 
