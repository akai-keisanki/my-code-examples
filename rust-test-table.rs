pub trait IRow
{
}

pub trait IQuery<Row: IRow>
{
    fn get() -> Vec<Row>;
}

pub trait ITable<Row: IRow>
{
    fn new() -> Self;
    fn add(&mut self, row: Row) -> ();
    fn query(&self) -> Query<Row>;
}

#[derive(Clone)]
pub struct Query<Row: IRow>
{
    rows: Vec<Row>
}

macro_rules! make_table
{
    {
        $table_name:ident
        {
            $($row_name:ident : $row_type:ty),*
        }
    }
    =>
    {
        mod $table_name
        {
            use crate::*;
           
            #[derive(Clone)]
            pub struct Row
            {
                pub id: usize,
                $(pub $row_name : $row_type),+
            }
       
            pub struct Table
            {
                rows: Vec<Row>
            }
           
            impl IRow for Row
            {
            }
           
            impl ITable<Row> for Table
            {
                fn new() -> Self
                {
                    Self { rows: Vec::<Row>::new() }
                }
           
                fn add(&mut self, row: Row) -> ()
                {
                    self.rows.push(row);
                }
           
                fn query(&self) -> Query<Row>
                {
                    Query { rows: self.rows.clone() }
                }
            }
           
            impl Table
            {
                pub fn add_row(&mut self, $($row_name : $row_type),*) -> ()
                {
                    self.add(Row { id: self.rows.len(), $($row_name),* });
                }
            }
        }
    }
}


make_table!{
    my_table
    {
        name: String
    }
}


fn main()
{
    println!("Hello, world!");
   
    let mut table = my_table::Table::new();
   
    table.add_row(String::from("Hello"));
    table.add_row(String::from("Hellow"));
    table.add_row(String::from("Helloo"));
   
    for x in table.query().rows
    {
        println!("{}: {}", x.id, x.name)
    }
}
