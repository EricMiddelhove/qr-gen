Simple wrapper for the qrcode-generator crate, making it available via stdin/out. 

Usage:

Install: 

In repo directory:
```
cargo install --path .
```


Example: 

```
   echo "apple.com" | qr-gen > qr-apple.png
```