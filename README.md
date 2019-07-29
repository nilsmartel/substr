# substr

substring can be used in to ways

## Get the first 4 letters from your stdout

`echo "Hello World" | substr 4`

## Get letters 2 to 5 from your stdout
`echo "Hello World" | subst 2 5`

and that's it folks!

Installing:
```
git clone https://github.com/sombrastudios/substr.git 
cd substr 
cargo build --release
cargo install --path .
```
