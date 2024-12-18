use goose::prelude::*;
use goose_eggs::{validate_and_load_static_assets, Validate};

async fn loadtest_index(user: &mut GooseUser) -> TransactionResult {
    // リクエストを送信
    let goose = user.get("/rust-wasm-github/").await?;

    // 検証条件を定義
    let validate = &Validate::builder()
        .status(200)
        .text("makinzm HP")
        .build();

    // 検証と静的リソースのロード
    validate_and_load_static_assets(user, goose, validate).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("LoadtestTransactions")
                .register_transaction(transaction!(loadtest_index))
        )
        .execute()
        .await?;

    Ok(())
}

