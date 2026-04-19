fn server_order(){}

mod module1{
    fn fix_incorrect_order(){
        book_order();
        super::server_order();
    }
    fn book_order(){}
}