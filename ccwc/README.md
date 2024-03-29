# Build Your Own wc Tool
This is my own version of the Unix command line tool wc. It is a solution to [John Crickett's Coding Challenges ](https://codingchallenges.fyi/challenges/challenge-wc).

ccwc counts various statistics of a text file, such as the number of lines, words, characters, and bytes. It takes command-line arguments to specify the operation and the file to be analyzed.

# Usage
```bash
Usage: ccwc [OPTIONS] [files]...

Arguments:
  [files]...  

Options:
  -c             The number of bytes in each input file is written to the standard output.
  -l             The number of lines in each input file is written to the standard output.
  -m             The number of characters in each input file is written to the standard output. If the current locale does not support multibyte characters, this is equivalent to the -c option.
  -w             The number of words in each input file is written to the standard output.
  -h, --help     Print help
  -V, --version  Print version
```

**Supported options:**

| Options | Description|
|---------|------------|
| `-l`    |Count the number of lines.|
| `-w`    |Count the number of words.|
| `-c`    |Count the number of bytes. This will cancel out any prior usage of the -m option.|
| `-m`    |Count the number of characters in the input file (same as -c if the current locale does not support multibyte characters).  This will cancel out any prior usage of the -c option.|
| `-h`    |Print help|
| `-V`    |Print version|

It also handles input from standard input (stdin) if no fileName is specified.

# How to Build
To build the application, run the following command:
```bash
$ cargo build --package ccwc --release
```
It will generate an optimized binary named `ccwc` in `target/release` directory.


# How to install
2. To Install `ccwc`, run following command:
   ```bash
   $ cargo install --path ccwc
   ```
3. Check whether you have `ccwc` installed correctly, open a shell and enter this line:
   ```bash
   $ ccwc --version
   ```
   You should see version number of ccwc
   ```bash
   ccwc 1.0.0
   ```
4. Great. ccwc is installed now. you can use ccwc directly in your terminal
   **Example**
   ```bash
   $ ccwc test.txt
     7145  58164 342190 test.txt
   ```
   

# How to run tests
To run test, run the following command:
```bash
$ cargo test --package ccwc
```

# How to run using cargo
To run the application using cargo, run the following command:
```bash
# Default behavior is to print the number of lines, words, and bytes of the input file.
$ cargo ccwc test.txt

# Read from standard input
$ cat test.txt | cargo ccwc
```

# Let's connect
* https://twitter.com/amalhanaja
* https://linkedin.com/in/amalhanaja
* https://www.tiktok.com/@amalhanaja
* https://www.youtube.com/@amalhanaja


