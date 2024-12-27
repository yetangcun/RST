use std::error::Error;
use clickhouse::{Client, Row};
use clickhouse::sql::Identifier;
use serde::{Deserialize, Serialize};

pub struct ClkHouseHdl {
    client: Client,
}

impl ClkHouseHdl {
    pub fn new(db_url: &str) -> Self {
        // 连接clickhouse的详细字符串示例，带账号密码和具体的数据库名称 
        // 连接示例: http://default:xiaoxiao@localhost:8123/blkdb
        // default: 账号
        // xiaoxiao: 密码 
        // localhost: 主机地址
        // 8123: 端口号
        // blkdb: 数据库名称
        // let client = Client::default().with_url(db_url); // db_url格式: http://username:password@localhost:8123/dbname
        let client = Client::default().with_url(format!("{}", db_url));
        ClkHouseHdl { client }
    }

    // 插入
    pub fn insert<T>(&self, sql:&str) -> Result<(), Box<dyn Error>> 
    where T: Row,
    {
        let _ = self.client.insert::<T>(sql);
        Ok(())
    }

    // 批量插入
    pub async fn inserts<T>(&self, sql:&str, rows: Vec<T>) -> Result<bool, Box<dyn Error>> 
    where T: Row + Serialize,
    {
        let mut insert = self.client.insert(sql)?;
        for row in rows {
            insert.write(&row).await?;
        }
        insert.end().await?;
        Ok(true)
    }

    // 更新
    pub async fn update(&self, sql:&str) -> Result<(), Box<dyn Error>>
    {
        // clickhouse不支持update操作
        // 需要使用 ALTER TABLE ... UPDATE 语法
        let _ = self.client.query(sql).execute().await?;
        Ok(())
    }

    // 删除
    pub async fn del(&self, sql:&str) -> Result<bool, Box<dyn Error>>
    {
        let _ = self.client.query(sql).execute().await?;
        Ok(true)
    }

    // 查询列表
    pub async fn querys<T>(&self, query: &str) -> Result<Vec<T>, Box<dyn Error>>
    where T: for<'b> Deserialize<'b> + Serialize + Row,
    {
        let res: Vec<T> = self.client.query(query).fetch_all().await?;
        Ok(res)
    }

    // 查询单个
    pub async fn query<T>(&self, query: &str) -> Result<T, Box<dyn Error>> 
    where T: for<'b> Deserialize<'b> + Serialize + Row,
    {
        let res: T = self.client.query(query).fetch_one().await?;
        Ok(res)
    }

    // 分页查询
    pub async fn query_page<T>(&self, sql: &str, pages: i32, sizes: i32) -> Result<(i32,Vec<T>), Box<dyn Error>>
    where T: for<'b> Deserialize<'b> + Serialize + Row,
    {
        // 计算偏移量
        let offset = (pages - 1) * sizes;
        
        // 构建分页SQL
        let page_sql = format!("{} LIMIT {} OFFSET {}", sql, sizes, offset);
        let count_sql = format!("SELECT count(*) as total FROM ({}) as t", sql);

        println!("{} \r\n {}", &page_sql, &count_sql);

        // 获取总记录数
        // let total: i32 = self.client.query(&count_sql)
        //     .fetch_one()
        //     .await?;
        
        let total: i32 = match self.client.query(&count_sql).fetch_one().await {
            Ok(result) => result,
            Err(e) => {
                println!("总记录数查询失败: {}, \r\n 请检查URL是否正确,例如:http://default:xiaoxiao@192.168.30.111:8123", e);
                return Err(Box::new(e));
            }
        };

        // 获取分页数据
        // let data: Vec<T> = self.client.query(&page_sql)
        //     .fetch_all()
        //     .await?;
        
        println!("{}", &page_sql);
        let data: Vec<T> = match self.client.query(&page_sql).fetch_all().await {
            Ok(result) => result,
            Err(e) => {
                println!("分页查询失败: {}, \r\n 请检查URL是否正确,例如:http://default:xiaoxiao@192.168.30.111:8123", e);
                return Err(Box::new(e));
            }
        };

        Ok((total,data))
    }
}