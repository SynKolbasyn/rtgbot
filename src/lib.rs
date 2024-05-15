mod bot;
mod types;


#[cfg(test)]
mod tests {
    use anyhow::Result;
    
    use crate::bot::Bot;

    
    #[test]
    fn it_works() -> Result<()> {
        let bot: Bot = Bot::new("");
        assert_eq!("", bot.token());
        Ok(())
    }
}
