# lambda-instance-beholder

Checks available instance types on Lambda GPU Cloud.

If you are from Lambda and don't want this to exists: Please message me via the issue tracker :)

## Usage

Prepare an `.env` file by renaming the `.example-env` file with: `mv .example-env .env`.  
Paste your credentials and a list of instance types of interest into the `.env` file.

To get your `SESSION_ID` login at [https://cloud.lambdalabs.com/login](https://cloud.lambdalabs.com/login) and open the dev tools of your browser. You'll find a cookie named `sessdionid` set by `lambdalabs.com`. It's value is your `SESSION_ID`.

Now run `cargo run` to start the program. It checks the availability of the instance types specified in `.env` and exits when one is available.

To get an audio feedback once an instance has been found, you can run a command to play a sound after the program exits. E.g. `cargo run && afplay /System/Library/Sounds/Glass.aiff`.
