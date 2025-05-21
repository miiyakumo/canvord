use di::injectable;
use sea_orm::DatabaseConnection;

pub trait DatabaseConnectionProvider {
    fn get_connection(&self) -> sea_orm::DatabaseConnection;
}

#[injectable(DatabaseConnectionProvider)]
pub struct DatabaseConnectionProviderImpl {
    connection: DatabaseConnection,
}

impl DatabaseConnectionProviderImpl {
    pub async fn new() -> Self {
        DatabaseConnectionProviderImpl { 
            connection: sea_orm::Database::connect(
                "mysql://root:Pass@word@canvord-sqldata:3306/text_item_api"
            ).await.unwrap(),
        }
    }
    
    async fn is_connected(&self) -> bool {
        self.connection.ping().await.is_ok()
    }
}

impl DatabaseConnectionProvider for DatabaseConnectionProviderImpl {
    fn get_connection(&self) -> DatabaseConnection {
        self.connection.clone()
    }
}