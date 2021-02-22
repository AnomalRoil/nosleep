Have you ever face this problem: you don't want your computer to fall asleep/send you to the lock screen every 2 minutes or so (maybe while working at home because of some pandemic and remote work policy), but you have a so-called "corporate" computer where some of the settings are not configurable by your user, because of "security" reasons? 

That's annoying, right? But what if you can still run arbitrary code/binaries?

Well, here's the easiest solution: let's run a software that would move the mouse/simulate a keypress every few minutes to avoid sleep :D 

Now, you might have some security concerns running arbitrary code on your device, so here's a small piece of software relying on very few dependencies that will press F13 every 2 minutes (yup, that's a possible keystroke even though you probably don't have a F13 key on your keyboard, which mean it's probably not going to break too many things or being recognized as a shortcut by too many other programs).

# To run

Requires Rust and Cargo.

```bash
git clone https://github.com/AnomalRoil/nosleep.git
cd nosleep
cargo run
```

