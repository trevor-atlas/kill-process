# kill-process

**Kill Process** is an alfred workflow and spiritual successor to the tenured [Kill Process](https://www.packal.org/workflow/kill-process) which I've used for many years.
I created this workflow because the original had a few kinks I disliked and it had a tendency to be slower than I liked.

It was also a great opportunity to learn more Rust 🦀 and build something blazing fast 🚀 <sub>/s</sub> (it is fast, though)

## FAQ
> I have a problem!

See [Caveats](#caveats). If that doesn't solve your problem feel free to open an issue with a detailed description of what you expected to happen, and what actually happened. :smile:

> Is it good?

Yes.

> Will it make me dinner?

No.

> Why did you write it like that?

I'm new to Rust, if you have suggestions or enhancements I'm happy to hear about them, but please be constructive <3.


## Screenshots

<p align="center">
  <img width="500" src="./screenshots/user-app.png?raw=true">
</p>

<p align="center">
  <img width="500" src="./screenshots/process.png?raw=true">
</p>


## Caveats
**Warning:** 🚨 I have only tested this on an M1 Macbook. I am cross compiling for intel macs so in theory everything should work fine for intel and native apple silicon macs (thanks, Rosetta!)
but if you encounter issues, please open a ticket (or a PR!)


I am not (and probably never will be) a verified Apple developer.
This means you will see a [SIP](https://developer.apple.com/documentation/security/disabling_and_enabling_system_integrity_protection) prompt when you run the workflow for the first time.

<p align="center">
  <img width="300" alt="SIP prompt" src="./screenshots/sip.png?raw=true">
</p>

You can get around this if you:
1. Click `Cancel`
2. Open `System Preferences -> Security & Privacy`
3. Click `Allow Anyway`.

<p align="center">
  <img width="500" alt="where to disable SIP and allow the workflow" src="./screenshots/update-security.png?raw=true">
</p>

After completing that, the next time you run the workflow you should see another (different) SIP prompt.

<p align="center">
  <img width="300" alt="The final SIP" src="./screenshots/final-sip.png?raw=true">
</p>

Click `Open` and you should be all set!


