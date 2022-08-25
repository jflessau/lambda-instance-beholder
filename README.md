# lambda-instance-beholder

Checks avaiable instance types on Lambda GPU Cloud.
Provide a list of relevant instance types and this program will log changes and play a "ping"-sound when availability changes.

## Usage

Prepare an `.env` file by renaming the `.example-env` file with: `mv .example-env .env`.
Paste your credentials and a list of instance types of interest into the `.env` file.

Run it with `cargo run`.
