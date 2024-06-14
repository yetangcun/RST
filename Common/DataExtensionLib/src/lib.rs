pub mod mysqlLib;
mod mssqlLib;
mod pgsqlLib;
mod mongodbLib;
mod clkhouseLib;
pub mod datasqlx;   

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait FrmRow: Sized {
    fn from_row(row: mysql::Row) -> Result<Self, mysql::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
