use std::collections::HashMap;

use crate::model::tick::Tick;

pub struct MarketEngine {

    market_state:

        HashMap<
            String,

            Tick
        >,
}

impl MarketEngine {

    pub fn new() -> Self {

        Self {

            market_state:

                HashMap::new()
        }
    }

    pub fn process_tick(

        &mut self,

        tick:
            Tick

    ) {

        self

            .market_state

            .insert(

                tick
                    .symbol
                    .clone(),

                tick
            );
    }

    pub fn get_market_state(

        &self

    )

    -> &HashMap<
            String,

            Tick
       >

    {

        &self
            .market_state
    }
}