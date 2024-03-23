# Kuuubedit

Powerful headless text editor for processing large files.

WARNING: Files are not buffered, the file size capable of being loaded is limited by your memory size.

## Args

### Mandatory

- `kuuubedit FILEPATH`: File to open.

### Optional

- `-u`, `--undo`: Enables the undo command.

## Commands

Use double quotes around values containing spaces. Use `\"` for a literal double quote.

- **Find**

    `f FIND_REGEX OUTPUT_FILE`: Initiate regex find operation and output results to file.

- **Replace**

    `r FIND_REGEX REPLACE_STRING`: Initiate regex replace operation on currently open file.

- **Replace Write**

    `rw FIND_REGEX REPLACE_STRING OUTPUT_FILE`: Initiate regex replace operation on currently open file and write result to file.

- **Write**

    `w OUTPUT_FILE`: Write current file to file.

- **Output**

    `o`: Output current file to stdout.

- **Undo**

    `u`: Undoes the last operation. Requires `--undo` arg.

- **Quit**

    `q`: Exits the program.
