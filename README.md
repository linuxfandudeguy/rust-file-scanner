# rust-file-scanner
> **Warning**: the file will randomly duplicate itself completely empty each time you use it. Each time you use it, please delete those files.




This is my first rust project yay :D

<div align="center">
<img src="https://skillicons.dev/icons?i=rust" alt="rust" height="40"/>
</div>

------------------------------------------------------------------------

This was made using **Cargo** and **Rust**.

This rust application scans for corrupted/empty files in a directory provided in command line arguments.

Here's a screenshot example:

![Screenshot 2024-08-12 9 46 38 PM](https://github.com/user-attachments/assets/c24cceb8-db46-4b2f-9bbc-40141f8fccd2)

> The project was still in development when the screenshot was taken.

If it finds files it will return the files with their file path and if not it won't do it, and it also displays a counter for the amount of files scanned.

The `Cargo.toml` file has the dependencies needed:

```toml
[dependencies]
walkdir = "2.3.2"
colored = "2.0.0"
```

Once you have downloaded necessary files/packages you can run:

```bash
cargo build
```
Then to scan a directory you can run:

```bash
cargo run ./
```

 anyone who's about to complain about I didn't give installation instructions for cargo too bad



