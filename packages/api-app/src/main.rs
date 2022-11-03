use prisma_client_rust::NewClientError;
use crate::prisma::{GrammaticalForm, new_client, PrismaClient};

mod prisma;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let client: PrismaClient = new_client().await.unwrap(); // Update on new release
    let user = client.user().create(
        vec![],
        false,
        GrammaticalForm::Indeterminate,
        vec![],
    ).exec().await.unwrap();
}
