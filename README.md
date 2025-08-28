# Pin - an easy way to quickly traverse your file system
Pin is a cli too used to quickly jump to specific locations by aliasing a path. 

### Important note
Currently only supports bash and linux systems. Requires use of the echo, cp and cd commands.

### Installation
To install pin, first clone the repository to your system
```
git clone https://github.com/Joe-Roser/pin.git PinInstall
```

Then, navigate into the directory
```
cd PinInstall
```

Finally, run the script
```
./install.sh
```

### Architecture
Pin uses a bash script to handle the changing of directories. Rust is used to make do the proccessing of the data and lookup, using a hashmap stored as a binary for fast serealisation and lookup. Exit codes are used to change how the bash script behaves on return.
You can see my original plan for the project [here](/plan.md).

return codes:
The rust program interfaces with the script above it by returning different return codes based on the desired program output:
 - 0: Success no response
 - 1: Error response
 - 2: Success cd

### Contributing
Anyone wishing to make improvements to the code or improve platform availability, I would love to hear from you. Just make a pull request
