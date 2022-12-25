# Mars
Mars (ma-rs) is an blazingly fast rust machine learning library. Simple and Powerful! 🦀🚀
# Contribution:
  Feel free to build this project. This is a public repository which will me guided by everyone!

# Examples:

**Gradient Descent**

Works fine. Green dots are the training data and red line is the models guesses.

![Screenshot_20221225_191458](https://user-images.githubusercontent.com/101834410/209475194-8ffa5f44-66f4-4c86-9703-2586f84e20b8.png)

try it by :
```
cargo run --example gradient_descent --release
```
-> It is **important** to use `--release` here, because training the model takes a lot more time compared to compilation time:

**So schenario goes like:**

without `--release` fast compilaton (+3 seconds) slow training (--much more than 3 seconds)

with `--release` slow compilation (-5 seconds) lot faster training (++much more time than 5 seconds)

