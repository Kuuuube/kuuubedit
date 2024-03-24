# Kuuubedit

Powerful headless text editor for processing enormous files using complex regex patterns.

## Args

### Mandatory

- `kuuubedit FILEPATH`: File to open.

### Optional

- `-u`, `--undo`: Enables the undo command and `--no-buf`.

- `-b BUFFER`, `--buffer BUFFER`: Sets file buffer size in bytes. Default: 100000000 (100MB), Minimum: 1024 (1KB), Maximum: 18446744073709551615 (18.4EB).

- `-n`, `--no-buf`: Disabled file buffering.

## Commands

Use double quotes around values containing spaces. Use `\"` for a literal double quote.

### Base Commands

- **Find**

    `f FIND_REGEX OUTPUT_FILE`: Initiate regex find operation and output results to file.

- **Replace**

    `r FIND_REGEX REPLACE_STRING OUTPUT_FILE`: Initiate regex replace operation on currently open file and write result to file.

- **Write**

    `w OUTPUT_FILE`: Write current file to file.

- **View**

    `v START_BYTE LENGTH`: Output string from file data of specified length in bytes starting from the specified byte.

    Invalid unicode may be outputted if a multi-byte UTF-8 character is split apart by the range specified. These characters are displayed using `�` and may not represent the full file data.

- **Quit**

    `q`: Exits the program.

### --no-buf only

- **Replace Active**

    `ra FIND_REGEX REPLACE_STRING`: Initiate regex replace operation on currently open file and overwrite loaded file data in memory with result.

- **Output**

    `o`: Output current file to stdout.

- **Undo**

    `u`: Undoes the last operation. Requires `--undo` arg.
