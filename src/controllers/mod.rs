mod instance0;
mod instance1;
mod instance2;
mod instance3;
mod instance4;
mod instance5;
mod instance6;
mod instance7;
mod instance8;
mod instance9;

use async_graphql::{
    MergedObject,
};


#[derive(MergedObject, Default, Clone)]
pub struct Query(
    instance0::EventInstanceQueries0,
    instance1::EventInstanceQueries1,
    instance2::EventInstanceQueries2,
    instance3::EventInstanceQueries3,
    instance4::EventInstanceQueries4,
    instance5::EventInstanceQueries5,
    instance6::EventInstanceQueries6,
    instance7::EventInstanceQueries7,
    instance8::EventInstanceQueries8,
    instance9::EventInstanceQueries9,
);


#[derive(MergedObject, Default, Clone)]
pub struct Mutation(
    instance0::EventInstanceMutations0,
    instance1::EventInstanceMutations1,
    instance2::EventInstanceMutations2,
    instance3::EventInstanceMutations3,
    instance4::EventInstanceMutations4,
    instance5::EventInstanceMutations5,
    instance6::EventInstanceMutations6,
    instance7::EventInstanceMutations7,
    instance8::EventInstanceMutations8,
    instance9::EventInstanceMutations9,
);
