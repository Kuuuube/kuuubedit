# Kuuubedit

Powerful headless text editor for processing enormous files. (Currently unfinished)

## Args

### Mandatory

- `kuuubedit FILE`: File to open.

### Optional

- `-u`, `--undo`: Enables the undo command.

## Commands

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
