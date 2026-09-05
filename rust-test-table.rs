pub trait IRow
{
}

pub trait IQuery<'a, Row: IRow>
{
    fn get_rows(&mut self) -> Vec<&'a mut Row>;
    fn get_first(&mut self) -> &'a mut Row;
}

pub trait ITable<'a, Row: IRow>
{
    fn new() -> Self;
    fn add(&mut self, row: Row) -> ();
    fn query(&mut self) -> Query<'a, Row>;
}

pub struct Query<'a, Row: IRow>
{
    rows: Vec<&'a mut Row>
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

            impl IQuery<Row> for Query<'_, Row>
            {
                fn get_rows(&mut self) -> Vec<&mut Row>
                {
                    self.rows.iter_mut().map(|x| &mut **x).collect()
                }

                fn get_first(&mut self) -> &mut Row
                {
                    self.rows[0]
                }
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
           
                fn query(&mut self) -> Query<'a, Row>
                {
                    Query { rows: self.rows.iter_mut().collect() }
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
        handle: String,
        name: String
    }
}


fn main()
{
    println!("Hello, world!");
   
    let mut table = my_table::Table::new();
   
    table.add_row(String::from("hello123"), String::from("Hello,"));
    table.add_row(String::from("w0rld"), String::from("World!"));
    table.add_row(String::from("interrobang"), String::from("?!"));
   
    for x in table.query().get_rows()
    {
        println!("{}: \"{}\" \"{}\"", x.id, x.handle, x.name)
    }
}
