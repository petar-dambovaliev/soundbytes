## soundbytes

The project has a goal in mind to allow the programmatic composition and playing of music. 
It includes an interpreter and a player.
The project is in very early stages so expect things to change without notice.

#### How to write the soundbytes script?

At the moment, there are few predefined functions and objects.

All combinations of notes, octaves and durations are available globally.

note: C  octave: 4 duration: 16th
`c_4_16`

note: C#  octave: 4 duration: 16th dotted
`c#_4_16*`

It is mandatory to set the tempo, at least once. 

`tempo(60);`

Creating a track from a bunch of notes.

Notice that the last couple of notes don't have an octave or a duration.

For the convenience of the user, if the octave and duration are identical, they can be omitted until a change is required.

`track(c_4_16, c#_4_16*, c_4_16, c, a, b)`

Playing the tracks

```
let bass = track(a_3_8*, b, g#_2_8*, a_3_8*, f_2_8*, f#, g, g);
let bass2 = track(x_8*, x, g#_2_16, b_3_16, d, f, e, c_2_16, e);
let solo = track(
        a_5_32, c, e, a, c, e, b, d,
        f, b, d, f, g#_4_32, b_5_32, d, g#_4_32,
        b_5_32, d, a, c, e, a, c, e,
        f_4_32, a_5_32, c, f_4_32, a_5_32, c, f#_4_32, a_5_32,
        c, f#_4_32, a_5_32, c, f, e, d, c,
        b, a, b, c, d, e, f, g
);

play(bass, bass2, solo);
```

The function `play` also accepts single notes but `notes` and `tracks` cannot be mixed together.  


Running the code
1. Compile the project and give the `.sb` source file to the binary

You can also checkout the test file in `/test`