# rgbcal: RGB LED calibration tool
Sarah Dylan

## Measurements

The optimal values were:
- **Red**: 15
- **Blue**: 11
- **Green**: 9
- **Framerate**: 60 was enough for me to not perceive the flickering but 90 was needed for the video

I did find hard to find a balance where the light was not too blue or too green. I noticed in the video It looked more 'pure' white than it did to my eye. In person, I would describe it as a cool white. I tried adjusting values to see if I could get warm white light, but I was not able to quite get there. 

## What I did and how it went
The wiring was pretty painless, and I just followed the example used in class. What I was shocked by was how incredibly easy embassy made the process of coding this project. I tried for while setting up embassy for the last assignment, and I kept failing. But I knew it would've been worth it, and I am glad I got to see it in action. Being able to 'spawn' different processes and then use mutexes to manage global state felt a lot more intuitive than our previous assignments. I think the abstraction made it a lot easier to wrap my head around what was happening. I did find the UI a little clunky to use as the double button press kind of necessitated a certain order you needed to adjust values (first red, then blue or green, and finally frame rate). I also think the program reading the value of the knob instead of adjusting based on changes from its current level was not great UI experience, but I also was not quite sure how would easily code that up.
