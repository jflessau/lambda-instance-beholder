# lambda-instance-beholder

Checks available instance types on Lambda GPU Cloud.  
Provide a list of relevant instance types and this program will log changes and play a "ping"-sound when availability changes.

If you are from Lambda Labs and don't want this to exists: Please message me via the issue tracker :)

## Usage

Prepare an `.env` file by renaming the `.example-env` file with: `mv .example-env .env`.  
Paste your credentials and a list of instance types of interest into the `.env` file.

To get your `SESSION_ID` login at [https://lambdalabs.com/cloud/login](https://lambdalabs.com/cloud/login) and open the dev tools of your browser. You'll find a cookie named `sessdionid` set by `lambdalabs.com`. It's value is your `SESSION_ID`.

Now run the code with `cargo run` and wait for instance types to get available.

## Credits

The "ping" sound effect was made by AlaskaRobotics and uploaded to Pixabay:  
[https://pixabay.com/de/sound-effects/service-bell-ring-14610/](https://pixabay.com/de/sound-effects/service-bell-ring-14610/)
