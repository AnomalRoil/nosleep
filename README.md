# No sleep

Have you ever face this problem: you don't want your computer to fall asleep/send you to the lock screen every 2 minutes or so (maybe while working at home because of some pandemic and remote work policy), but you have a so-called "corporate" computer where some of the settings are not configurable by your user, because of "security" reasons? 

That's annoying, right? But what if you can still run arbitrary code/binaries?

Well, here's the easiest solution: let's run a software that would move the mouse/simulate a keypress every few minutes to avoid sleep :D 

Now, you might have some security concerns running arbitrary code on your device, so here's a small piece of software relying on very few dependencies that will press F13 every 2 minutes (yup, that's a possible keystroke even though you probably don't have a F13 key on your keyboard, which mean it's probably not going to break too many things or being recognized as a shortcut by too many other programs).

# To run it

Requires Rust and Cargo.

```bash
git clone https://github.com/AnomalRoil/nosleep.git
cd nosleep
cargo run
```

# Disclaimer

Any security professional will totally condemn any tentative to circumvent corporate security policies that might enforce early locking of your devices.
These are in place to ensure your password is required to access corporate information and your data. You should never forget to lock your device whenever you walk away from it, otherwise you're making the evil maid's job super easy. Please don't do that and lock your device whenever you're not in front of it...

But as somone who's obligated to use multiple computers at once, I know early locking can be super annoying in some use-cases, so here you are. 

You're responsible for your own choices, yada yada, I won't be held responsible for anything that happens to your computer, yada yada.

Full disclaimer: I haven't reviewed the code of the dependencies I'm using, this is probably totally vulnerable to any kind of supply chain attack, and could totally already be compromised. That being said, this was classed under "accepted risks" in my threat model, YMMV. 

Enjoy living a dangerous life in a dangerous world.
