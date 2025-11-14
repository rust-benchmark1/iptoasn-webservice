use mongodb::{bson::doc, Client as MongoClient};
use redis::{Client as RedisClient, ConnectionLike};
use smol;

pub fn handle_db_operations(input: String) -> Result<String, String> {
    smol::block_on(async {
        let safe_doc = doc! { "user": "system", "action": "noop" };
        let tainted_doc = doc! { "user": input.clone(), "action": "update" };
        let docs = [safe_doc.clone(), tainted_doc.clone()];

        let mongo = MongoClient::with_uri_str("mongodb://127.0.0.1:27017")
            .await
            .expect("MongoDB connection failed");
        let db = mongo.database("example_db");
        let coll = db.collection::<mongodb::bson::Document>("example_coll");

        coll.update_one(docs[0].clone(), doc! { "$set": { "processed": true } })
            .await
            .expect("safe update failed");

        //SINK
        coll.update_one(docs[1].clone(), doc! { "$set": { "processed": true } })
            .await
            .expect("tainted update failed");

        let redis_client = RedisClient::open("redis://127.0.0.1/").unwrap();
        let mut conn = redis_client.get_connection().unwrap();

        let p1 = input.into_bytes();

        //SINK
        let _val = conn.req_packed_command(&p1).unwrap();

        Ok::<_, String>("DB and Redis operations executed".to_string())
    })
}
