use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn github(ctx: Context<'_>) -> Result<(), Error> {
    let mut message = "https://github.com/Whisky-App/Whisky".to_owned();
    let cmd_name = ctx.invoked_command_name();
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