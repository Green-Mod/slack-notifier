# Slack notifier

> Just a simple CLI tool to send notifications to Slack.

Please note that this project is just a playground to start learning Rust, it is not intended to be used in production.

## Prerequisites

* [Slack App](https://api.slack.com/messaging/webhooks)
* [Rust build environment](https://www.rust-lang.org/tools/install)

## Usage

After cloning the repository, you should setup a `.env` file based on the `.env.example` file. Inside it you should specify the `SLACK_WEBHOOK_URL` variable.

Once the environment is ready we can build the binary using Cargo:

    $ cargo build --release

Then we should find the CLI binary in the `target/release` directory:

    $ ls target/release
    slack-notifier

Finally, we can run the binary:

    $ ./target/release/slack-notifier --help
