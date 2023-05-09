use async_graphql::{
    MergedObject,
    Object,
};

#[derive(MergedObject, Default, Clone)]
pub struct Query(
    EventInstanceQueries,
);


#[derive(Default, Clone)]
pub struct EventInstanceQueries;

#[Object]
impl EventInstanceQueries{
    async fn say_hi(
        &self,
    ) -> Result<&'static str, anyhow::Error> {
        Ok("hi")
    }
}

#[derive(MergedObject, Default, Clone)]
pub struct Mutation(
    EventInstanceMutations,
);

#[derive(Default, Clone)]
pub struct EventInstanceMutations;

#[Object]
impl EventInstanceMutations{
    async fn create(
        &self,
    ) -> Result<bool, anyhow::Error> {
        Ok(true)
    }
}