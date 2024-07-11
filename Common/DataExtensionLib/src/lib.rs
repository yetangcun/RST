pub mod mysqlLib;
mod cfg;
mod mssqlLib;
mod pgsqlLib;
mod mongodbLib;
mod clkhouseLib;
pub mod datasqlx;   
use sqlx::sqlite::{SqliteRow};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


// 测试实现
pub trait test_trat {
    fn test_fn(&self) -> String;
    fn test_fn2(&self) -> String {
        return "test_fn2".to_string();
    }
}

// pub trait SqliteRw: for<'r> sqlx::FromRow<'r, SqliteRow> {}
// impl<T> SqliteRw for T where T: for<'r> sqlx::FromRow<'r, SqliteRow> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
