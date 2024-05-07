use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, aliases("epic", "gog", "amazon", "prime"))]
pub async fn heroic(ctx: Context<'_>) -> Result<(), Error> {

    let epic = "";
    let gog = "";
    let amazon = "";

    let bigmessage = "";

    let mut message = "".to_owned();
    let cmd_name = ctx.invoked_command_name();

    if cmd_name == "heroic" {
        message += bigmessage;
    }
    else if cmd_name == "epic" {
        message += epic;
    }
    else if cmd_name == "gog" {
        message += gog;
    }
    else if cmd_name == "amazon" || cmd_name == "prime" {
        message += amazon;
    }

    if let Context::Prefix(prefix) = ctx {
        match prefix.msg.clone().referenced_message {
            Some(parent) => {
                message += "\n\nThis command was invoked by ";
                message += ctx.author().to_string().as_str();

                parent.reply_ping(&ctx, message).await?;
                prefix.msg.delete(ctx).await?;
            },
            None => {
                ctx.reply(message).await?;
            }
        }
    } else {
        ctx.reply(message).await?;
    }

    Ok(())
}