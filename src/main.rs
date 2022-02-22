use clap::Parser;
use dotenv::dotenv;
use slack_hook::{AttachmentBuilder, PayloadBuilder, Slack};
use std::env;

#[derive(Parser, Debug)]
#[clap(author = "Giorgio Bertolotti <giorgio@greenmod.it>")]
#[clap(version = "0.1.0")]
#[clap(about = "A simple Slack notifier utility")]
struct Args {
    /// Title to display in the notification
    #[clap(short, long, default_value = "Title")]
    title: String,

    /// Text to send in Slack's notification
    #[clap(short, long)]
    message: String,

    /// Indicates if the notification is a success or a failure
    #[clap(short, long)]
    success: bool,

    /// Bot username to display in Slack
    #[clap(short, long, default_value = ":rocket:")]
    icon: String,
}

fn main() {
    let args = Args::parse();
    dotenv().ok();

    let slack_url = env::var("SLACK_WEBHOOK_URL").unwrap_or_else(|_| {
        eprintln!("Please specify the SLACK_WEBHOOK_URL environment variable");
        std::process::exit(1);
    });

    match args {
        Args {
            title,
            message,
            success,
            icon,
        } => {
            println!("Sending Slack notification...");

            let slack = Slack::new(&slack_url[..]).unwrap();

            let p = PayloadBuilder::new()
                .text(title)
                .icon_emoji(icon)
                .attachments(vec![AttachmentBuilder::new(message)
                    .color(if success { "#3d9970" } else { "#b13d41" })
                    .build()
                    .unwrap()])
                .build()
                .unwrap();

            let res = slack.send(&p);
            match res {
                Ok(()) => println!("Notification sent!"),
                Err(x) => {
                    eprintln!("Error sending notification: {:?}", x);
                    std::process::exit(1);
                }
            }
        }
    }
}
