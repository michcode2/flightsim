i made a flight simulator!!
the flight dynamics suck and idk how the performance will be on your computer but it works good for me
ive only tested mac arm and mac x86 because im not at my PC

"but natalie, my computer is linux/32 bit/linux 32 bit!!"
so what you need to do is install cargo from https://rust.sh and then cargo build --release and then it will work for you
for linux users, you should know how to do this

the stall speed of the plane is about 40m/s
when you land the plane, if you land at more than -1m/s vertically, the program will quit and tell you that it landed too fast
otherwise it will congradulate you

you will get a log.json. it can be used in the jupyter notebook to be a bit of a black box if you want
you have to replace the final , in the log with ]}. i am too lazy to fix this.

"but natalie, when i roll and then pitch a bunch the plane goes fucky!!" the control system is unphysical, so this is just an effect of this
in a normal plane, you move the elevator to push the nose up, if youve not got enough speed/control to get the nose up, it doesnt go up.
here, the nose (and other controls) just goes up no matter what so i think thats why it gets a bit fucky, future releases will address this
for any nerds in the comments, this is why you dont see phugoid, SPO, dutch roll and all that. all will be fixed in time.

if you have any issues with this email me: natalie.kf@outlook.com