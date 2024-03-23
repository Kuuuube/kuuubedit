# Kuuubedit

Powerful headless text editor for processing enormous files.

## Args

### Mandatory

- `kuuubedit FILEPATH`: File to open.

### Optional

- `-u`, `--undo`: Enables the undo command.

- `-b BUFFER`, `--buffer BUFFER`: Sets file buffer size (unfinished, currently, only --no-buf is supported).

## Commands

Use double quotes around values containing spaces. Use `\"` for a literal double quote.

### Base Commands

- **Find**

    `f FIND_REGEX OUTPUT_FILE`: Initiate regex find operation and output results to file.

- **Replace Write**

    `rw FIND_REGEX REPLACE_STRING OUTPUT_FILE`: Initiate regex replace operation on currently open file and write result to file.

- **Write**

    `w OUTPUT_FILE`: Write current file to file.

- **Quit**

    `q`: Exits the program.

### --no-buf only

- **Replace**

    `r FIND_REGEX REPLACE_STRING`: Initiate regex replace operation on currently open file.

- **Output**

    `o`: Output current file to stdout.

- **Undo**

    `u`: Undoes the last operation. Requires `--undo` arg.
