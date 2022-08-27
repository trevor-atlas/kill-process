# kill-process

**Kill Process** is an alfred workflow and spiritual successor to the tenured [Kill Process](https://www.packal.org/workflow/kill-process) which I've used for many years.
I created this workflow because the original had a few kinks I disliked and it had a tendency to be slower than I liked.

It was also a great opportunity to learn more Rust 🦀!



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


2. I am not (and probably never will be) a verified Apple developer.
This means you will see a [SIP](https://developer.apple.com/documentation/security/disabling_and_enabling_system_integrity_protection) prompt when you run the workflow for the first time.

<p align="center">
  <img width="300" alt="SIP prompt" src="./screenshots/sip.png?raw=true">
</p>

If you click `Cancel` and open `System Preferences -> Security & Privacy` you can get around this by clicking `Allow Anyway`.

<p align="center">
  <img width="500" alt="where to disable SIP and allow the workflow" src="./screenshots/update-security.png?raw=true">
</p>

After completing that, the next time you run the workflow you should see another (different) SIP prompt. Click `Open` and you should be all set!

<p align="center">
  <img width="300" alt="The final SIP" src="./screenshots/final-sip.png?raw=true">
</p>


