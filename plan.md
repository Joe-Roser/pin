# Plan

### interface:
<alias> - jump to alias
--add <alias> <path> - add the alias to the path
--list - list all aliases
--update <alias> - opens an interactive tui to change the alias
--help - lists all commands and usage
--delete <alias> - delete an alias
====maybe====
--tag <alias> - tags an alias
Some kind of frequency data or rececy data

### Data flow:
user input ->
captured by the function and passed straight to the rust program

command selector ->
parses the arguments and passes them to the correct function

command ->
Possibly read the map into memory
Validates any paths given
Transforms the input to the target, possibly returning
Possibly save the map to disk

Exit ->
Exit with the correct error code for the task

Bash ->
Act based on exit code

### Model
Hashmap<string, PinAlias>
PinAlias struct{
    path: string/PathBuf - Not decided yet
    tags: Vec<string> - if I implement tags
}

Todo:
- [ ] Tags


