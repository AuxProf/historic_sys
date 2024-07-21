use uuid::Uuid;
use crate::traits::crud::Crud;

pub struct User {
    id: Uuid,
    email: String,
}


impl Crud for User {
    
}