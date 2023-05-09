mod instance1;
mod instance2;
mod instance3;
mod instance4;
mod instance5;

use async_graphql::{
    MergedObject,
};


#[derive(MergedObject, Default, Clone)]
pub struct Query(
    instance1::EventInstanceQueries1,
    instance2::EventInstanceQueries2,
    instance3::EventInstanceQueries3,
    instance4::EventInstanceQueries4,
    instance5::EventInstanceQueries5,
);


#[derive(MergedObject, Default, Clone)]
pub struct Mutation(
    instance1::EventInstanceMutations1,
    instance2::EventInstanceMutations2,
    instance3::EventInstanceMutations3,
    instance4::EventInstanceMutations4,
    instance5::EventInstanceMutations5,
);
