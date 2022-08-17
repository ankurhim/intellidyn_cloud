use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{ Client, Error };

#[derive(Debug)]
pub struct DynamodbClient {
    pub db_client: Client
}

impl DynamodbClient {
    pub async fn init() -> Result<Self, Error> {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        Ok(DynamodbClient {
            db_client: client
        })
    }

    pub async fn check_tables(&self, table: &str) -> Result<bool, Error> {
        let resp = self.db_client.list_tables().send().await?;
        let names = resp.table_names().unwrap_or_default();

        let mut table_exists = false;
        for name in names {
            println!("{:?}", &name);
            if name == table {
                table_exists = true
            }
        }

        Ok(table_exists)
    }
}