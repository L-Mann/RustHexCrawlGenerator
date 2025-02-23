## Using the generator:
```
./Hexcrawlin filename.md Lower-Left-Row Lower-Left-Column Upper-Right-Row Upper-Right-Column
```
**filename.md** = the name of the output file, recommended to use the .md file extension, but I'm not your dad.  
**Lower-Left-Row** = The row value of the hexmap's leftmost and lowest point  
**Lower-Left-Column** = The column value of the hexmap's leftmost and lowest point  
**Lower-Left-Row** = The row value of the hexmap's rightmost and highest point  
**Lower-Left-Row** = the column value of the hexmap's rightmost and highest point

The generator will generate the hexes in a rectangular pattern based on the parameters given to it on the command line.  

Inputting weird CLAs will cause strange behavior. I didn't add much error checking.  

## Sources used

All table values and entries are taken directly from [here](https://lukegearing.blot.im/wolves-upon-the-coast-hexfill-procedure)  
