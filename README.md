
# zm_web_proc

**zm_web_proc** is a Rust-based tool that automates the processing of ZoneMinder export files. The tool extracts a ZIP file, processes video files using `zm_vidsort` and `mp4_to_webm`, and moves the converted `.webm` files to a `videos` directory.

## Features

- **ZIP Extraction**: Unzips the specified ZoneMinder export file.
- **Video Sorting**: Uses `zm_vidsort` to organize video files.
- **Video Conversion**: Converts `.mp4` files to `.webm` format using the `mp4_to_webm` tool.
- **File Management**: Moves `.webm` files to a `videos` directory.

## Requirements

- **zm_vidsort**: The program uses the `zm_vidsort` tool for organizing video files.
  - GitHub repo: [zm_vidsort](https://github.com/nestorwheelock/zm_vidsort)
- **mp4_to_webm**: The program uses `mp4_to_webm` to convert `.mp4` files to `.webm` format.
  - GitHub repo: [mp4_to_webm](https://github.com/nestorwheelock/mp4_to_webm)
- **unzip**: The tool uses `unzip` to extract ZIP files.
- **Rust toolchain**: You need Rust installed to compile and run the program.

Make sure both **zm_vidsort** and **mp4_to_webm** are installed and available in your system path.

### Installing Prerequisites

1. **zm_vidsort**:
   Clone and install the `zm_vidsort` tool:
   ```bash
   git clone https://github.com/nestorwheelock/zm_vidsort.git
   cd zm_vidsort
   cargo build --release
   ```

2. **mp4_to_webm**:
   Clone and install the `mp4_to_webm` tool:
   ```bash
   git clone https://github.com/nestorwheelock/mp4_to_webm.git
   cd mp4_to_webm
   cargo build --release
   ```

3. **unzip**:
   Install `unzip` using your system's package manager:
   ```bash
   sudo apt-get install unzip  # For Debian-based systems
   ```

## Installation

### Step 1: Clone the Repository

```bash
git clone https://github.com/nestorwheelock/zm_web_proc.git
cd zm_web_proc
```

### Step 2: Build the Project

```bash
cargo build --release
```

## Usage

To use **zm_web_proc**, specify a ZoneMinder export ZIP file as an argument. The tool will unzip the file, process the video files, and move the converted `.webm` files to a `videos` directory.

### Command

```bash
./target/release/zm_web_proc <zm_export_zip_file>
```

### Example

```bash
./target/release/zm_web_proc zm_export_12345.zip
```

This will:

1. Unzip the `zm_export_12345.zip` file.
2. Organize video files using `zm_vidsort`.
3. Convert `.mp4` files to `.webm`.
4. Move the `.webm` files to the `videos` directory in the current working directory.

### Output

- The processed `.webm` files will be moved to a `videos` directory in the current working directory.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
