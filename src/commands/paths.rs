use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, aliases("whiskywine", "bottles", "logs", "whiskycmd", "whats_where", "whatswhere", "where", "path"))]
pub async fn paths(ctx: Context<'_>) -> Result<(), Error> {

    let whiskywine = "**WhiskyWine:** `~/Library/Application Support/com.isaacmarovitz.Whisky/Libraries`";
    let bottles = "**Bottles:** `~/Library/Containers/com.isaacmarovitz.Whisky/Bottles`";
    let logs = "**Logs:** `~/Library/Logs/com.isaacmarovitz.Whisky`";
    let whiskycmd = "**WhiskyCmd:** `/usr/local/bin/whisky`";

    let findernote = "Use `⌘ + Shift + .` to show hidden files. `com.isaacmarovits.Whisky` may show up as `Whisky`.

";

    let bigmessage = "Need to find something installed by Whisky? Look in the following locations.

`Note: If you're looking in Finder, you may need to do ⌘ + Shift + . to show hidden files. Also, com.isaacmarovitz.Whisky may just show up as a folder titled Whisky.`
";

    let mut message = "".to_owned();
    let cmd_name = ctx.invoked_command_name();

    if cmd_name == "path" || cmd_name == "paths" || cmd_name == "whats_where" || cmd_name == "whatswhere" || cmd_name == "where" {
        message += bigmessage;
        message += "\n";
        message += whiskywine;
        message += "\n";
        message += bottles;
        message += "\n";
        message += logs;
        message += "\n";
        message += whiskycmd;
    }
    else if cmd_name == "whiskywine" {
        message += findernote;
        message += whiskywine;
    }
    else if cmd_name == "bottles" {
        message += findernote;
        message += bottles;
    }
    else if cmd_name == "logs" {
        message += findernote;
        message += logs;
    }
    else if cmd_name == "whiskycmd" {
        message += findernote;
        message += whiskycmd;
    }

    if let Context::Prefix(prefix) = ctx {
        match prefix.msg.clone().referenced_message {
            Some(parent) => {
                message += &format!("\n-# This command was invoked by {} using `~{}`", ctx.author().to_string().as_str(), cmd_name);

                parent.reply_ping(&ctx, message).await?;
                prefix.msg.delete(ctx).await?;
            },
            None => {
                message += &format!("\n-# This command was invoked using `~{}`", cmd_name);
                ctx.reply(message).await?;
            }
        }
    } else {
        ctx.reply(message).await?;
    }

    Ok(())
}