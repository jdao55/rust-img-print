# rust-img-print
Print images to 24 bit termials as text 
 
 ![image created](tux_example.png?raw=true "") 
  
# Building
1. clone repository 
```
git clone https://github.com/jdao55/img-print.git && cd img-print
```
2. Build using cargo
```
cargo build --release
```
Execuatable can be found at 
```
rust-img-print/target/release/img-print
```
# Usage 
```
img-print [-g] <filename>
img-print [-g] <filename> <output-width> <output-height>
img-print [-g] <filename> <output-width> <output-height> <output-char>
img-print (-h | --help)
```

## Options
```-h --help```          Print help message 
 
```-g --greyscale```   Output in greyscale
# Example
Tested with Konsole
![image created](2b_sample_output.png?raw=true "") 
 
