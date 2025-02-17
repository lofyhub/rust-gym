# LOC (Lines of Code)

A simple application for counting lines of code in a directory. It tracks the following:

- Source code lines
- Number of comments
- Blank lines
- Number of Files

This project is a learning exercise to explore file system traversal, parsing, and Rust development. 

If you want some battle test LOC (Lines of Code) Rust alterantive check out [https://github.com/cgag/loc](https://github.com/cgag/loc)

## Features

- Supports counting lines of code across 116 programming languages.
- Differentiates between source code, comments, and blank lines

## Setup

### 1. Clone the repository

```bash
git clone https://github.com/lofyhub/rust-gym.git
cd loc
```

### 2. Build the project

Once inside the project directory, use Cargo to build the project:

```bash
cargo build
```

### 3. Run the project

To run the project, use the following command:

```bash
cargo run
```
If you want to target a specific directory, you can change the path in line `52` of the `src/lib.rs` file to the desired directory.

## Folder Structure

```
loc/
├── src/
│   ├── main.rs    # The main entry point of the application
│   ├── lib.rs     # library functions
│   └── utils.rs    # Utility functions for the project
├── Cargo.toml     # Project dependencies and metadata
└── README.md      # This file
```

# Benchmark Results

This is the benchmarkd performed against the [TensorFlow GitHub repository](https://github.com/tensorflow/tensorflow).

I know theire might be arears to improve the performance

## Benchmark Summary

| Language          | Files | Blank Spaces | Lines of Code | Comments |
|-------------------|-------|--------------|---------------|----------|
| Go                | 41    | 3922         | 32210         | 31166    |
| XML               | 55    | 361          | 1583          | 0        |
| CMake             | 42    | 193          | 1269          | 815      |
| Rust              | 3     | 19           | 256           | 10       |
| C#                | 2     | 75           | 289           | 22       |
| INI               | 2     | 6            | 53            | 0        |
| Python            | 3121  | 181556       | 926250        | 102953   |
| Objective-C++     | 53    | 1097         | 5446          | 660      |
| Protobuf          | 268   | 4455         | 12129         | 11806    |
| Ada               | 1     | 1            | 385           | 0        |
| YAML              | 16    | 54           | 3092          | 31       |
| Objective-C       | 21    | 348          | 1244          | 395      |
| HTML              | 235   | 17686        | 322245        | 29       |
| C/C++ Header      | 5533  | 126436       | 480462        | 192630   |
| Swift             | 18    | 249          | 1325          | 634      |
| TOML              | 1     | 1            | 6             | 0        |
| D                 | 25    | 51           | 101           | 0        |
| C++               | 6     | 173          | 922           | 182      |
| Batch             | 13    | 114          | 429           | 0        |
| Lock              | 30    | 14           | 95            | 2        |
| Markdown          | 1169  | 32078        | 100326        | 8360     |
| JSON              | 57    | 20           | 1881          | 57       |
| C                 | 29    | 353          | 1683          | 225      |
| CSS               | 2     | 21           | 94            | 2        |
| Perl              | 2     | 36           | 150           | 41       |
| Plain Text        | 98    | 2089         | 19940         | 1623     |
| JavaScript        | 1     | 2            | 59            | 3        |
| Bourne Shell      | 247   | 2200         | 9333          | 6346     |
| Java              | 179   | 4292         | 27694         | 1214     |

**Total Time Taken:** Reading all files done in **2.814686625s**

Please note that this time may vary depending on your PC specifications, memory usage, and background programs.

I recommend running this tool on a large GitHub repository and sharing the results for further insights.

## Notes

- Dot files, files without read permissions, and files without extensions are **not counted** in the analysis.
- Additionally, we only check for files that match our existing programming language file extensions. 
- If you want to see which programming languages we allow (116), you can refer to the `src/utils.rs` file.
- Feel free to use this for learning purposes only, and not for any critical applications. Enjoy!

## Contributing

Feel free to fork this repo, create a branch, and submit a pull request!

## License

[MIT License](LICENSE)