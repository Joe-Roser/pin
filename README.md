# Pin - an easy way to quickly traverse your file system
Pin is a cli too used to quickly jump to specific locations by aliasing a path. 

### Architecture:
Pin uses a bash script to handle the changing of directories. Rust is used to make do the proccessing of the data and lookup, using a hashmap stored as a binary for fast serealisation and lookup. Exit codes are used to change how the bash script behaves on return.

