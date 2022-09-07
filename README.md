```
git clone https://github.com/clnoll/libgit2-config-env-var.git
cd libgit2-config-env-var
cargo build --release
GIT_CONFIG_GLOBAL=.gitconfig_test_env_var ./target/release/libgit2-config-env-var  # prints "In custom config section" to stdout
./target/release/libgit2-config-env-var  # prints "Error: config value 'myconfigsection.custom' was not found" to stdout
```
