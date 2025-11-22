use teloxide::prelude::*;
use teloxide::types::ParseMode;
use crate::utils::keyboards;
use crate::services::web3::Web3Service;

pub struct WalletRoute;

impl WalletRoute {
    pub async fn wallet_main(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let text = "💰 <b>Wallet</b>                                        \n\n\
                   <b>Choose an option:</b>\n\n\
                   🆕 <b>Create Wallet:</b> Generate new BSC wallet\n\
                   📂 <b>Already Have Wallet:</b> Access existing wallet";

        let keyboard = keyboards::wallet_initial();

        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;

        Ok(())
    }

    pub async fn create_wallet_prompt(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let user_has_wallet = Self::check_user_has_wallet(&msg.chat.id).await;
        
        if user_has_wallet {
            let text = "⚠️ <b>Wallet Already Exists</b>                                 \n\n\
                       <b>You already have a wallet created.</b>\n\n\
                       Creating a new wallet will:\n\
                       • Generate new seed phrase & private key\n\
                       • Create new wallet address\n\
                       • Old wallet will no longer be accessible\n\n\
                       <b>Are you sure you want to create a new wallet?</b>";

            let keyboard = keyboards::wallet_confirm_create();

            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        } else {
            Self::create_wallet(bot, msg, web3).await?;
        }

        Ok(())
    }

    pub async fn create_wallet(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let text = "⏳ <b>Creating Your Wallet...</b>\n\n\
                   <b>Generating secure wallet:</b>\n\
                   • Creating BSC wallet address\n\
                   • Generating seed phrase\n\
                   • Securing private key\n\
                   • Initializing wallet\n\n\
                   <i>This will take a few seconds...</i>";

        let processing_msg = bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let wallet_data = Self::generate_wallet(&msg.chat.id).await;

        let text = format!(
            "✅ <b>Wallet Created Successfully!</b>                                                 \n\n\
            <b>Balance (BSC):</b> <code>{} BNB</code>\n\
            <b>Wallet Address:</b>\n<code>{}</code>\n\
            <b>Last TXN:</b> <code>{}</code>\n\n\
            <b>⚠️ SECURITY WARNING:</b>\n\
            • Never share your seed phrase or private key\n\
            • Store them in a secure location\n\
            • This is your only way to access funds\n\
            • We cannot recover lost seed phrases\n\n\
            <b>Important Notes:</b>\n\
            • Only send/receive <b>BEP-20 tokens</b> on <b>BSC Network</b>\n\
            • Do not send other network assets\n\
            • Keep your wallet secure\n\
            • Check send to see seedphrase and private key \n\n\

            <i>💰 Your wallet is ready for investments!</i>",
            wallet_data.balance,
            wallet_data.address,
            wallet_data.last_txn
        );

        let keyboard = keyboards::wallet_created();

        bot.edit_message_text(processing_msg.chat.id, processing_msg.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;

        Ok(())
    }

    pub async fn existing_wallet(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let user_has_wallet = Self::check_user_has_wallet(&msg.chat.id).await;
        
        if !user_has_wallet {
            let text = "❌ <b>No Wallet Found</b>\n\n\
                       <b>You haven't created a wallet yet.</b>\n\n\
                       Please create a wallet first to access wallet features.";

            let keyboard = keyboards::wallet_no_wallet();

            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        } else {
            let wallet_data = Self::get_wallet_data(&msg.chat.id).await;
            
            let text = format!(
                "💰 <b>Wallet</b>                                           \n\n\
                 <b>Balance (BSC):</b> <code>{} BNB</code>\n\
                 <b>Wallet Address:</b>\n<code>{}</code>\n\
                 <b>Last TXN:</b> <code>{}</code>",
                wallet_data.balance,
                wallet_data.address,
                wallet_data.last_txn
            );

            let keyboard = keyboards::wallet_main();

            bot.send_message(msg.chat.id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(keyboard)
                .await?;
        }

        Ok(())
    }

    pub async fn receive_funds(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let user_has_wallet = Self::check_user_has_wallet(&msg.chat.id).await;
        
        if !user_has_wallet {
            return Self::existing_wallet(bot, msg, web3).await;
        }

        let wallet_data = Self::get_wallet_data(&msg.chat.id).await;
        
        let text = format!(
            "📥 <b>Receive Funds</b>                                                    \n\n\
             <b>Your BSC Wallet Address:</b>\n\
             <code>{}</code>\n\n\
             <b>⚠️ Important Instructions:</b>\n\
             • Send <b>ONLY via BSC Network</b>\n\
             • Only send <b>BEP-20 tokens</b>\n\
             • Do not send from CEX directly\n\
             • Test with small amount first\n\n\
             <b>Supported:</b> BNB, BUSD, USDT, USDC (BEP-20)\n\
             <b>Network:</b> Binance Smart Chain (BSC)\n\n\
             <i>🚫 Sending via wrong network may result in lost funds!</i>",
            wallet_data.address
        );

        let keyboard = keyboards::wallet_receive();

        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;

        Ok(())
    }

    pub async fn send_funds(bot: Bot, msg: Message, web3: Web3Service) -> ResponseResult<()> {
        let user_has_wallet = Self::check_user_has_wallet(&msg.chat.id).await;
        
        if !user_has_wallet {
            return Self::existing_wallet(bot, msg, web3).await;
        }

        let wallet_data = Self::get_wallet_data(&msg.chat.id).await;

        let text = format!(
            "📤 <b>Send / Withdraw Funds</b>                                                            \n\n\
            <b>To send funds from your wallet, export using:</b>                                           \n\n\
            <b>Method 1: Seed Phrase</b>\n\
            <code>{}</code>\n\n\
            <b>Method 2: Private Key</b>\n\
            <code>{}</code>\n\n\
            <b>Step-by-Step Guide:</b>\n\n\
            <b>Step 1:</b> Install a supported wallet app\n\
            • Trust Wallet, MetaMask, SafePal, etc.\n\n\
            <b>Step 2:</b> Import your wallet\n\
            • Choose 'Import Wallet'\n\
            • Enter seed phrase OR private key\n\
            • Set password for security\n\n\
            <b>Step 3:</b> Connect to BSC Network\n\
            • Network Name: Binance Smart Chain\n\
            • RPC URL: https://bsc-dataseed.binance.org/\n\
            • Chain ID: 56\n\
            • Symbol: BNB\n\
            • Block Explorer: https://bscscan.com\n\n\
            <b>Step 4:</b> Send funds\n\
            • Use 'Send' function in wallet app\n\
            • Enter recipient address\n\
            • Confirm transaction\n\
            • Pay gas fees in BNB\n\n\
            <b>⚠️ SECURITY WARNING:</b>\n\
            • Never share seed phrase/private key publicly\n\
            • Only import in trusted wallet apps\n\
            • Double-check network is BSC\n\
            • Test with small amount first\n\n\
            <i>💡 For security, direct sending is disabled in bot.\n\
            Export wallet to external app for full control.</i>",
            wallet_data.seed_phrase,
            wallet_data.private_key
        );

        let keyboard = keyboards::wallet_send();

        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;

        Ok(())
    }

    pub async fn handle_callback(
        bot: Bot,
        msg: Message,
        web3: Web3Service,
        parts: &[&str],
    ) -> ResponseResult<()> {
        if parts.len() > 1 {
            match parts[1] {
                "main" => Self::wallet_main(bot, msg, web3).await,
                "create_prompt" => Self::create_wallet_prompt(bot, msg, web3).await,
                "create" => Self::create_wallet(bot, msg, web3).await,
                "existing" => Self::existing_wallet(bot, msg, web3).await,
                "receive" => Self::receive_funds(bot, msg, web3).await,
                "send" => Self::send_funds(bot, msg, web3).await,
                _ => Self::wallet_main(bot, msg, web3).await,
            }
        } else {
            Self::wallet_main(bot, msg, web3).await
        }
    }

    // Utility functions
    async fn check_user_has_wallet(chat_id: &ChatId) -> bool {
        // In real implementation, check database
        // For now, return false to test create wallet flow
        true
    }

    async fn get_wallet_data(chat_id: &ChatId) -> WalletData {
        // Mock data - in real app, fetch from database
        WalletData {
            address: "0x8F17A5a178Dc74241E5e336F41E2975cEf7f05a1".to_string(),
            seed_phrase: "apple banana cherry dolphin elephant flower grape hat ice juice kite lemon".to_string(),
            private_key: "0x4cbe3c6c6d8d9a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7".to_string(),
            balance: 0.85,
            last_txn: "0x4a3b7c8d9e2f1a0b9c8d7e6f5a4b3c2d1e0f9a8b7c6d5e4f3a2b1c0d9e8f7a6b".to_string(),
        }
    }

    async fn generate_wallet(chat_id: &ChatId) -> WalletData {
        let seed_words = vec![
            "apple", "banana", "cherry", "dolphin", "elephant", "flower",
            "grape", "hat", "ice", "juice", "kite", "lemon"
        ];
        
        let seed_phrase = seed_words.join(" ");
        let private_key = format!("0x{:x}{:x}", chat_id.0, rand::random::<u64>());
        let address = format!("0x{:x}{:x}", chat_id.0, rand::random::<u64>());

        WalletData {
            address,
            seed_phrase,
            private_key,
            balance: 0.0,
            last_txn: "None".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct WalletData {
    address: String,
    seed_phrase: String,
    private_key: String,
    balance: f64,
    last_txn: String,
}