# Search in Rust project's dependency

The vscode's global search can not search in dependency, so i make this simple tool to wrokaround this problem.

Actually this project olny contains a few lines of code, which invoke `cargo metadata` and print manifest of each package. The searching works is done by other tools.

## Ripgrep

Text of regex search using [`ripgrep`](https://github.com/BurntSushi/ripgrep): 

```shell
rg  'Unknown error' $(dep)
```



## Ast-Grep

Structual search using [`ast-grep`](https://github.com/ast-grep/ast-grep): 

```shell
sg -p 'pub fn description($$$) -> $$$' -l rust $(dep)
```
