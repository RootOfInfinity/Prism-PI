# Prism Private Intelligence
## A unique learn-to-code app
#### Brought to you by William Campbell and Graydon Cory

This app was originally made for the Congressional App Challenge, the link
to the original introduction video is here: <https://youtu.be/bwtudXcAAko>
This app was submitted on Oct 30th 2025, and the video was completed on the
same day.

### Who This App Is For
Prism Private Intelligence is a transition app. It is meant to transition
people from skills in block coding to skills in text coding. Many of the
skills you use in block based languages are directly used in a text-based
environment, but it is hard to get past the mental hurdle of switching
formats so easily, as you now need to worry about typos, grammar errors,
and sometimes typechecking. Many people do block coding when they are
young, but never get into computer science, and we think this is for that
reason. While the app doesn't yet support this full functionality, the main
framework for the challenge based learning, the custom programming language,
and rudimentary prototype blocks are completed.

### Why You Should Care
Many people see the Learn-To-Code movement as 'dead', for a multitude
of reasons.  One reason is that our learning tools we use are not built
to actually learn programming.  In many frameworks, they offer blocks and
javascript, in where you build it in either blocks or JS and can transfer
them in between at any time. From first-hand experience, this does not work.
A young person will build their project in blocks, get curious at the button,
press it, and immedietly go back because it seems crazy complicated. If you
are a programmer, you know that it is not more complex than the block code
they already wrote, but it is not easy to see that. Thus, it requires a more
sophisticated approach, with a 'Mediated Transfer' between blocks and text
to completely understand it.

### Why We Went With Open Source
Open source is a vital part of the wider software ecosystem that is often
overlooked.  Many of the most important and influental projects are open
source, as that lets it get to as many people as possible. While we may not
make any money on this project, if even just 1 more person was positively
influenced by this app, it would be worth it.  Nevertheless, this was a great
learning experience for both of us, and we want to have our work free for
anyone to have. Also, the Slint GUI framework required us to put it as GPLv3
or sign up for some sort of thing, so that was a big push towards copyleft.

### The Tools For The Job
This program was made in the Rust Programming Language, a memory-safe low-level
language that is very fun to write in. Its enums and pattern matching saved
the internal typechecker. We used the Slint GUI framework, which was very
useful in quickly making the frontend we needed for this project. However,
it might not have been the best choice when making the block-based language,
so there is plans to switch frontends to a game engine. Also invaluable
was the json-rust library, which quickly and efficently parsed our json,
so we could easily write tests for the story mode section.


### Thank You!
From the bottom of our hearts, thank you for reading this README and supporting
this project. If you feel inclined, give us a star! Fork us! Send an issue
or pull request if you want! We can't promise we'll work on this forever,
but we care about feedback.
