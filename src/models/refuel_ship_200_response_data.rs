/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RefuelShip200ResponseData {
    #[serde(rename = "agent")]
    pub agent: Box<models::Agent>,
    #[serde(rename = "fuel")]
    pub fuel: Box<models::ShipFuel>,
    #[serde(rename = "transaction")]
    pub transaction: Box<models::MarketTransaction>,
}

impl RefuelShip200ResponseData {
    pub fn new(agent: models::Agent, fuel: models::ShipFuel, transaction: models::MarketTransaction) -> RefuelShip200ResponseData {
        RefuelShip200ResponseData {
            agent: Box::new(agent),
            fuel: Box::new(fuel),
            transaction: Box::new(transaction),
        }
    }
}

